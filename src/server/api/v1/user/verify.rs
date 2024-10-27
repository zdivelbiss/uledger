use crate::{
    postmark,
    server::{internal_error, AppState, UserSession},
    util::{EmailAddress, VerificationToken},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", routing::post(create))
        .route("/", routing::delete(delete))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user does not exist")]
    UserNotExists,

    #[error("email already in use")]
    EmailInUse,

    #[error("no verification match")]
    NoVerificationMatch,

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
                Self::NoVerificationMatch => StatusCode::NOT_FOUND,
                Self::Database(error) => internal_error(error),
                Self::Postmark(error) => internal_error(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
struct CreateInfo {
    email_address: EmailAddress,
}

#[axum::debug_handler]
async fn create(
    user_session: UserSession,
    state: State<AppState>,
    create_info: Json<CreateInfo>,
) -> Result<()> {
    let user_id = user_session.get_id().await;
    let email_address = &create_info.email_address;
    let verification_token = VerificationToken::gen();

    query!(
        "
        INSERT INTO users.email_verification
                (id, email_address, proof_token)
            VALUES
                ($1, $2, $3)
            ON CONFLICT (id)
                DO UPDATE SET
                    created = NOW(),
                    email_address = $2,
                    proof_token = $3
        ;
        ",
        user_id,
        email_address.as_str(),
        (&verification_token).to_string()
    )
    .execute(state.db())
    .await?;

    let transaction = postmark::Transaction::verification(
        &create_info.email_address,
        chrono::Utc::now(),
        verification_token,
    );

    postmark::send(transaction).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct DeleteInfo {
    email_address: EmailAddress,
    proof_token: VerificationToken,
}

#[axum::debug_handler]
async fn delete(
    user_session: UserSession,
    app_state: State<AppState>,
    delete_info: Json<DeleteInfo>,
) -> Result<()> {
    let user_id = user_session.get_id().await;
    let email_address = &delete_info.email_address;
    let proof_token = &delete_info.proof_token;

    let rows_affected = query!(
        "
        DELETE FROM users.email_verification
            WHERE
                id = $1
                    AND
                email_address = $2
                    AND
                proof_token = $3
        ;
        ",
        user_id,
        email_address.as_str(),
        proof_token.to_string()
    )
    .execute(app_state.db())
    .await?
    .rows_affected();

    match rows_affected {
        1 => {
            query!(
                "
                UPDATE users.account
                    SET
                        email_address = $2,
                        email_verified_on = $3
                    WHERE
                        id = $1
                ;
                ",
                user_id,
                email_address.as_str(),
                chrono::Utc::now()
            )
            .execute(app_state.db())
            .await?;

            Ok(())
        }

        0 => Err(Error::NoVerificationMatch),

        rows_affected => {
            error!("Dropped {rows_affected} during email validation!");

            Ok(())
        }
    }
}
