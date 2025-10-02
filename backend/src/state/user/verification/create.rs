use crate::util::VerificationToken;
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
    fn from(error: sqlx::Error) -> Self {
        let Some(db_error) = error.as_database_error() else {
            return Self::Database(error);
        };

        match (db_error.code().as_deref(), db_error.constraint()) {
            (Some("23505"), Some("email_verification_email_address_key")) => Error::EmailInUse,

            _ => Self::Database(error),
        }
    }
}

impl super::UserVerification {
    pub async fn create(&self, user_id: Uuid, email_address: &str) -> Result<(), Error> {
        let verification_token = VerificationToken::generate();
        let expiry = chrono::Utc::now() + chrono::Duration::hours(1);

        query!(
            "
            UPDATE _user.profile
                SET
                    pending_email_address = $2,
                    pending_email_address_token = $3,
                    pending_email_address_expiry = $4
                WHERE
                    id = $1
            ;
            ",
            user_id as _,
            email_address as _,
            verification_token.as_bytes(),
            expiry
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
