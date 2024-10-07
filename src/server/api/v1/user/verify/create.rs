use crate::{
    server::{get_user_id, internal_error, state::AppState, NoUserIdError},
    util::{EmailAddress, VerificationToken},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json,
};
use tower_sessions::Session;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/", post(create))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    NotAuthenticated(#[from] NoUserIdError),

    #[error("user does not exist")]
    UserNotExists,

    #[error("email already in use")]
    EmailInUse,

    #[error(transparent)]
    Database(sqlx::Error),

    #[error(transparent)]
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
    fn into_response(self) -> Response {
        Response::builder()
            .status(match self {
                Error::NotAuthenticated(_) => StatusCode::UNAUTHORIZED,
                Error::UserNotExists => StatusCode::NOT_FOUND,
                Error::EmailInUse => StatusCode::CONFLICT,
                Error::Database(error) => internal_error(error),
                Error::Postmark(error) => internal_error(error),
            })
            .body(axum::body::Body::empty())
            .unwrap()
    }
}

#[derive(Debug, serde::Deserialize)]
struct RequestBody {
    email_address: EmailAddress,
}

#[axum::debug_handler]
async fn create(
    session: Session,
    state: State<AppState>,
    body: Json<RequestBody>,
) -> Result<StatusCode, Error> {
    use crate::postmark::{send_email, Transaction};

    let token: VerificationToken = VerificationToken::gen();
    let email_address = &body.email_address;
    let user_id = get_user_id(&session).await?;

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
