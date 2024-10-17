use crate::{server::state::user::Role, util::EmailAddress};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use uuid::Uuid;

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

impl super::UserState {
    pub async fn register(
        &self,
        display_name: &str,
        email_address: &EmailAddress,
        password: &str,
    ) -> Result<(Uuid, String), Error> {
        let salt = SaltString::generate(&mut rand::rngs::OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .serialize();

        let user = query!(
            "
            INSERT INTO auth.users (role, email, password_salt, password_hash, display_name)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, display_name
            ;
            ",
            i16::from(Role::Regular),
            email_address.as_str(),
            salt.as_str(),
            password_hash.as_str(),
            display_name
        )
        .fetch_one(&self.db)
        .await?;

        Ok((user.id, user.display_name))
    }
}
