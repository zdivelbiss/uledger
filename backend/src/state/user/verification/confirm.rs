use crate::util::VerificationToken;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no verification match")]
    NoMatch,

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Postmark(#[from] crate::postmark::Error),
}

impl super::UserVerification {
    pub async fn confirm(
        &self,
        user_id: Uuid,
        verification_token: VerificationToken,
    ) -> Result<(), Error> {
        let rows_affected = query!(
            "
            UPDATE _user.profile
                SET
                    email_address = pending_email_address,
                    pending_email_address = NULL,
                    pending_email_address_token = NULL,
                    pending_email_address_expiry = NULL
                WHERE
                    id = $1
                        AND
                    pending_email_address_token = $2
                        AND
                    pending_email_address_expiry > NOW()
            ;
            ",
            user_id,
            verification_token.as_bytes()
        )
        .execute(&self.db)
        .await?
        .rows_affected();

        match rows_affected {
            1 => Ok(()),

            0 => Err(Error::NoMatch),

            rows_affected => unreachable!("Unexpected {rows_affected} finishing email validation!"),
        }
    }
}
