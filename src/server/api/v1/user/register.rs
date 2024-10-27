use crate::{
    server::{
        api::UserAccess,
        htmx::{hx_redirect, IsHtmx},
        AppState,
    },
    util::EmailAddress,
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
    fn into_response(self) -> axum::response::Response {
        use crate::server::{internal_error, internal_error_dbg};

        axum::response::Response::builder()
            .status(match &self {
                Self::DuplicateEmail => StatusCode::CONFLICT,
                Self::Database(error) => internal_error(error),
                Self::PasswordHashing(error) => internal_error_dbg(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Info {
    display_name: String,
    email_address: EmailAddress,
    password: String,
}

#[axum::debug_handler]
pub async fn handler(
    state: State<AppState>,
    is_htmx: IsHtmx,
    form: Form<Info>,
) -> Result<Response, Error> {
    let display_name = form.display_name.as_str();
    let email_address = form.email_address.as_str();
    let password = form.password.as_str();

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .serialize();

    let salt = salt.as_str();
    let password_hash = password_hash.as_str();

    query!(
        "
        INSERT INTO users.account
                (access, email_address, password_salt, password_hash, display_name)
            VALUES
                ($1, $2, $3, $4, $5)
        ;
        ",
        UserAccess::Regular as _,
        email_address,
        salt,
        password_hash,
        display_name
    )
    .execute(state.db())
    .await?;

    // TODO login

    if *is_htmx {
        Ok([hx_redirect("/login")].into_response())
    } else {
        Ok(().into_response())
    }
}
