use crate::util::{EmailAddress, VerificationToken};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user does not exist")]
    UserNotExists,

    #[error("email already in use")]
    EmailInUse,

    #[error("internal database error")]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        let Some(db_err) = err.as_database_error() else {
            return Self::Database(err);
        };

        match (db_err.code().as_deref(), db_err.constraint()) {
            (Some("23503"), Some("email_verification_user_id_fkey")) => Error::UserNotExists,
            (Some("23505"), Some("email_verification_email_address_key")) => Error::EmailInUse,

            _ => Self::Database(err),
        }
    }
}

impl super::UserState {
    #[instrument(skip(self))]
    pub async fn create_email_verification(
        &self,
        user_id: Uuid,
        email_address: &EmailAddress,
    ) -> Result<VerificationToken, Error> {
        let token = VerificationToken::gen();

        query!(
            "
            INSERT INTO auth.email_verification (user_id, email_address, proof_token)
                VALUES ($1, $2, $3)
                ON CONFLICT (user_id) DO UPDATE SET
                    user_id = $1,
                    created = NOW(),
                    email_address = $2,
                    proof_token = $3
            ;
            ",
            user_id,
            email_address.as_str(), // TODO figure out why email_address won't coerce
            token.to_string()
        )
        .execute(self.pool())
        .await?;

        Ok(token)
    }
}
