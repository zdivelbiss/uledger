use crate::{
    server::{
        htmx::{hx_redirect, is_htmx},
        internal_error,
        state::AppState,
    },
    util::EmailAddress,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Form,
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
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(match &self {
                Self::AlreadyLoggedIn => StatusCode::CONFLICT,
                Self::InvalidLogin => StatusCode::UNAUTHORIZED,
                Self::Database(error) => internal_error(error),
                Self::PasswordHashing(error) => internal_error(error),
                Self::Session(error) => internal_error(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Info {
    email_address: EmailAddress,
    password: String,
}

#[axum::debug_handler]
pub async fn handler(
    state: State<AppState>,
    session: Session,
    headers: HeaderMap,
    form: Form<Info>,
) -> Result<Response, Error> {
    if !session.is_empty().await {
        return Err(Error::AlreadyLoggedIn);
    }

    let email_address = form.email_address.as_str();
    let password = form.password.as_str();

    let user = query!(
        "
        SELECT id, password_salt, password_hash, display_name FROM auth.users
            WHERE email = $1
        ;
        ",
        email_address
    )
    .fetch_one(state.db())
    .await?;

    let password_salt = SaltString::from_b64(&user.password_salt)?;
    let calculated_hash = Argon2::default()
        .hash_password(password.as_bytes(), &password_salt)?
        .serialize();

    // check password ...
    if user.password_hash.as_str() != calculated_hash.as_str() {
        return Err(Error::InvalidLogin);
    }

    // update session ...
    session.insert("user_id", user.id).await?;
    let user_agent = headers.get("User-Agent").and_then(|v| v.to_str().ok());
    session.insert("user_agent", user_agent).await?;
    session.insert("display_name", user.display_name);

    Ok(if is_htmx(&headers) {
        [hx_redirect("/")].into_response()
    } else {
        ().into_response()
    })
}
