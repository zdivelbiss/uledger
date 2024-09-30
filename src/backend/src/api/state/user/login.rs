use crate::util::{EmailAddress, PasswordDigest};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user does not exist")]
    InvalidEmail,

    #[error("incorrect password provided")]
    InvalidPassword,

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
            _ => Self::Database(err),
        }
    }
}

impl super::UserState {
    pub async fn login(
        &self,
        email_address: &EmailAddress,
        password_digest: &PasswordDigest,
    ) -> Result<Uuid, Error> {
        let user = query!(
            "
            SELECT id, password_salt, password_hash FROM auth.users
                WHERE email = $1
            ;
            ",
            email_address.as_str()
        )
        .fetch_one(self.pool())
        .await?;

        let password_salt = SaltString::from_b64(&user.password_salt)?;
        let calculated_hash = Argon2::default()
            .hash_password(password_digest.as_slice(), &password_salt)?
            .serialize();

        if calculated_hash.as_str() != user.password_hash.as_str() {
            Err(Error::InvalidPassword)
        } else {
            Ok(user.id)
        }
    }
}
