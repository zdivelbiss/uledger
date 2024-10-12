use crate::{
    server::{internal_error, state::AppState},
    util::{EmailAddress, VerificationToken},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tower_sessions::Session;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user does not exist")]
    UserNotExists,

    #[error("email already in use")]
    EmailInUse,

    #[error("internal server error")]
    Database(sqlx::Error),

    #[error("internal server error")]
    Postmark(#[from] crate::postmark::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        let Some(db_err) = err.as_database_error() else {
            return Self::Database(err);
        };

        match (db_err.code().as_deref(), db_err.constraint()) {
            (Some("23503"), Some("email_verification_user_id_fkey")) => Error::UserNotExists,
            (Some("23505"), Some("email_verification_email_address_key")) => Error::EmailInUse,

            _ => Self::Database(err),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(match &self {
                Self::UserNotExists => StatusCode::NOT_FOUND,
                Self::EmailInUse => StatusCode::CONFLICT,
                Self::Database(error) => internal_error(error),
                Self::Postmark(error) => internal_error(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Info {
    email_address: EmailAddress,
}

#[axum::debug_handler]
pub async fn handler(
    session: Session,
    state: State<AppState>,
    body: Json<Info>,
) -> Result<StatusCode, Error> {
    use crate::postmark::{send_email, Transaction};

    let token: VerificationToken = VerificationToken::gen();
    let email_address = &body.email_address;
    let user_id = crate::server::state::get_user_id(&session).await;

    query!(
        "
        INSERT INTO auth.email_verification (user_id, email_address, proof_token)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id) DO UPDATE SET
                user_id = $1,
                created = NOW(),
                email_address = $2,
                proof_token = $3
        ;
        ",
        user_id,
        email_address.as_str(), // TODO figure out why email_address won't coerce
        token.to_string()
    )
    .execute(state.db())
    .await?;

    send_email(Transaction::verification(
        &body.email_address,
        chrono::Utc::now(),
        token,
    ))
    .await?;

    Ok(StatusCode::OK)
}
