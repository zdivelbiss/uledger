use crate::EmailAddress;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use rand::rngs::OsRng;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user already exists")]
    DuplicateEmail,

    #[error(transparent)]
    Database(sqlx::Error),

    #[error(transparent)]
    PasswordHash(#[from] argon2::password_hash::Error),
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

impl super::User {
    pub async fn register(
        &self,
        email_address: &EmailAddress,
        password: &str,
        display_name: &str,
    ) -> Result<(), Error> {
        let salt_string = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)?
            .serialize();

        query!(
            "
            INSERT INTO _user.account
                    (access, email_address, password_salt, password_hash, display_name)
                VALUES
                    ($1, $2, $3, $4, $5)
            ;
            ",
            super::UserAccess::Regular as _,
            email_address.as_str(),
            salt_string.as_str(),
            password_hash.as_str(),
            display_name
        )
        .execute(self.db())
        .await?;

        Ok(())
    }
}
