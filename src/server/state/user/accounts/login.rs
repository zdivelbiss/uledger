use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use lib::EmailAddress;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid email & password combination")]
    InvalidCredentials,

    #[error(transparent)]
    Database(sqlx::Error),

    #[error(transparent)]
    PasswordHash(#[from] argon2::password_hash::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::InvalidCredentials,
            error => Self::Database(error),
        }
    }
}

impl super::UserAccounts {
    pub async fn login(&self, email_address: &EmailAddress, password: &str) -> Result<Uuid, Error> {
        let user = query!(
            "
            SELECT id, password_salt, password_hash
                FROM _user.account
                WHERE
                    email_address = $1
            ;
            ",
            email_address.as_str()
        )
        .fetch_one(self.db())
        .await?;

        let password_salt = SaltString::from_b64(&user.password_salt)?;
        let calculated_hash = Argon2::default()
            .hash_password(password.as_bytes(), &password_salt)?
            .serialize();

        // check password ...
        if user.password_hash.as_str() != calculated_hash.as_str() {
            return Err(Error::InvalidCredentials);
        }

        Ok(user.id)
    }
}
