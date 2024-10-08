use crate::server::{
    responses::{email_in_use, internal_error},
    state::AppState,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user already exists")]
    DuplicateEmail,

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
            (Some("23505"), Some("users_email_key")) => Error::DuplicateEmail,

            _ => Self::Database(err),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::DuplicateEmail => email_in_use().into_response(),

            Error::PasswordHashing(error) => internal_error(error).into_response(),
            Error::Database(error) => internal_error(error).into_response(),
        }
    }
}

#[axum::debug_handler]
pub async fn register(
    state: State<AppState>,
    form: Form<super::AuthInfo>,
) -> Result<StatusCode, Error> {
    let email_address = &form.email_address;
    let password = &form.password;

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .serialize();

    query!(
        "
        INSERT INTO auth.users (role, email, password_salt, password_hash)
            VALUES ($1, $2, $3, $4)
        ;
        ",
        i16::from(super::Role::Regular),
        email_address.as_str(),
        salt.as_str(),
        password_hash.as_str()
    )
    .execute(state.db())
    .await?;

    // TODO potentially log user in as well?

    Ok(StatusCode::OK)
}
