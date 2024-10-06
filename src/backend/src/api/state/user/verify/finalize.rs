use crate::util::{EmailAddress, VerificationToken};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal database error")]
    Database(sqlx::Error),
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

impl super::super::UserState {
    pub async fn finalize_email_verification(
        &self,
        user_id: Uuid,
        email_address: &EmailAddress,
        proof_token: &VerificationToken,
    ) -> Result<bool, sqlx::Error> {
        let rows_affected = query!(
            "
            DELETE FROM auth.email_verification
                WHERE (user_id, email_address, proof_token) = ($1, $2, $3)
            ;
            ",
            user_id,
            email_address.as_str(),
            proof_token.to_string()
        )
        .execute(self.pgpool())
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Ok(false);
        } else if rows_affected > 1 {
            warn!("Dropped {rows_affected} during email validation!");

            return Ok(false);
        }

        query!(
            "
            UPDATE auth.users
                SET
                    email = $2,
                    email_verified_on = $3
                WHERE id = $1
            ;
            ",
            user_id,
            email_address.as_str(),
            chrono::Utc::now().date_naive()
        )
        .execute(self.pgpool())
        .await?;

        Ok(true)
    }
}
