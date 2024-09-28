use crate::util::EmailAddress;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user already exists")]
    UserExists,

    #[error("failed to hash password")]
    PasswordHashing(argon2::password_hash::Error),

    #[error("internal database error")]
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

impl super::UserState {
    pub async fn register_user(
        &self,
        email_address: &EmailAddress,
        password_digest: &[u8; 512],
    ) -> Result<Uuid, Error> {
        use argon2::{
            password_hash::{rand_core::OsRng, SaltString},
            Argon2, PasswordHasher,
        };

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password_digest, &salt)?
            .serialize();

        let record = query!(
            "
            INSERT INTO auth.users (role, email, salt, password_hash)
                VALUES ($1, $2, $3, $4)
                RETURNING auth.users.id
            ;
            ",
            <&str>::from(super::Role::Regular),
            email_address.as_str(),
            salt.as_str(),
            password_hash.as_str()
        )
        .fetch_one(self.pool())
        .await?;

        Ok(record.id)
    }
}
