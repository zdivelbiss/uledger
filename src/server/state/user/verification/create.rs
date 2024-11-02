use crate::postmark;
use crate::{EmailAddress, VerificationToken};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("email already in use")]
    EmailInUse,

    #[error(transparent)]
    Database(sqlx::Error),

    #[error(transparent)]
    Postmark(#[from] crate::postmark::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        let Some(db_err) = err.as_database_error() else {
            return Self::Database(err);
        };

        match (db_err.code().as_deref(), db_err.constraint()) {
            (Some("23505"), Some("email_verification_email_address_key")) => Error::EmailInUse,

            _ => Self::Database(err),
        }
    }
}

impl super::UserVerification {
    pub async fn create(&self, user_id: Uuid, email_address: &EmailAddress) -> Result<(), Error> {
        let verification_token = VerificationToken::gen();

        query!(
            "
            INSERT INTO _user.email_verification
                    (id, email_address, proof_token)
                VALUES
                    ($1, $2, $3)
                ON CONFLICT (id)
                    DO UPDATE SET
                        created = NOW(),
                        email_address = $2,
                        proof_token = $3
            ;
            ",
            user_id,
            email_address as _,
            verification_token.to_string()
        )
        .execute(&self.db)
        .await?;

        let transaction = postmark::Transaction::verification(
            email_address,
            chrono::Utc::now(),
            verification_token,
        );

        postmark::send(transaction).await?;

        Ok(())
    }
}
