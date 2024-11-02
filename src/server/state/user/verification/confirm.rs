use crate::{EmailAddress, VerificationToken};
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
        email_address: &EmailAddress,
        proof_token: VerificationToken,
    ) -> Result<(), Error> {
        let rows_affected = query!(
            "
            DELETE FROM _user.email_verification
                WHERE
                    id = $1
                        AND
                    email_address = $2
                        AND
                    proof_token = $3
            ;
            ",
            user_id,
            email_address.as_str(),
            proof_token.to_string()
        )
        .execute(&self.db)
        .await?
        .rows_affected();

        match rows_affected {
            1 => {
                query!(
                    "
                    UPDATE _user.profile
                        SET
                            email_address = $2,
                            email_verified_on = $3
                        WHERE
                            id = $1
                    ;
                    ",
                    user_id,
                    email_address as _,
                    chrono::Utc::now()
                )
                .execute(&self.db)
                .await?;

                Ok(())
            }

            0 => Err(Error::NoMatch),

            rows_affected => unreachable!("Dropped {rows_affected} confirming email validation!"),
        }
    }
}
