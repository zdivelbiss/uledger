use crate::server::{internal_error, state::AppState};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user already exists")]
    UserExists,

    #[error("failed to hash password")]
    PasswordHashing(argon2::password_hash::Error),

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::PasswordHashing(err)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        let Some(db_err) = err.as_database_error() else {
            return Self::Database(err);
        };

        match (db_err.code().as_deref(), db_err.constraint()) {
            (Some("23505"), Some("users_email_key")) => Error::UserExists,

            _ => Self::Database(err),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(match &self {
                Error::UserExists => StatusCode::CONFLICT,
                Error::PasswordHashing(error) => internal_error(error),
                Error::Database(error) => internal_error(error),
            })
            .body(axum::body::Body::empty())
            .unwrap()
    }
}

#[axum::debug_handler]
pub async fn register(
    state: State<AppState>,
    body: Json<super::AuthInfo>,
) -> Result<StatusCode, Error> {
    let email_address = &body.email_address;
    let password_digest = &body.password_digest;

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password_digest.as_slice(), &salt)?
        .serialize();

    query!(
        "
        INSERT INTO auth.users (role, email, password_salt, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING auth.users.id
        ;
        ",
        <&str>::from(crate::util::Role::Regular),
        email_address.as_str(),
        salt.as_str(),
        password_hash.as_str()
    )
    .fetch_one(state.db())
    .await?;

    Ok(StatusCode::OK)
}
