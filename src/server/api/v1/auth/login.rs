use crate::server::{internal_error, state::AppState};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use tower_sessions::Session;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user already logged in")]
    AlreadyLoggedIn,

    #[error("invalid email & password combination")]
    InvalidLogin,

    #[error(transparent)]
    Database(sqlx::Error),

    #[error("failed to hash password")]
    PasswordHashing(argon2::password_hash::Error),

    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::PasswordHashing(err)
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::InvalidLogin,
            error => Self::Database(error),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(match &self {
                Error::AlreadyLoggedIn => StatusCode::CONFLICT,
                Error::InvalidLogin => StatusCode::UNAUTHORIZED,
                Error::PasswordHashing(error) => internal_error(error),
                Error::Database(error) => internal_error(error),
                Error::Session(error) => internal_error(error),
            })
            .body(axum::body::Body::empty())
            .unwrap()
    }
}

#[axum::debug_handler]
pub async fn login(
    state: State<AppState>,
    session: Session,
    headers: HeaderMap,
    body: Json<super::AuthInfo>,
) -> Result<StatusCode, Error> {
    if !session.is_empty().await {
        return Err(Error::AlreadyLoggedIn);
    }

    let email_address = &body.email_address;
    let password_digest = &body.password_digest;

    let user = query!(
        "
        SELECT id, password_salt, password_hash FROM auth.users
            WHERE email = $1
        ;
        ",
        email_address.as_str()
    )
    .fetch_one(state.db())
    .await?;

    let password_salt = SaltString::from_b64(&user.password_salt)?;
    let calculated_hash = Argon2::default()
        .hash_password(password_digest.as_slice(), &password_salt)?
        .serialize();

    // check password ...
    if user.password_hash.as_str() != calculated_hash.as_str() {
        return Err(Error::InvalidLogin);
    }

    // update session ...
    session.insert("user_id", user.id).await?;
    let user_agent = headers.get("User-Agent").and_then(|v| v.to_str().ok());
    session.insert("user_agent", user_agent).await?;

    Ok(StatusCode::OK)
}
