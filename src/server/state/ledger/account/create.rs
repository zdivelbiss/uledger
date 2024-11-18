use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("already exists")]
    Duplicate,

    #[error("name too long")]
    NameLength,

    #[error("description too long")]
    DescriptionLength,

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        let Some(db_error) = error.as_database_error() else {
            return Self::Database(error);
        };

        match db_error.constraint() {
            Some("account_unq") => Self::Duplicate,
            Some("account_chk_name_len") => Self::NameLength,
            Some("account_chk_description_len") => Self::DescriptionLength,

            _ => Self::Database(error),
        }
    }
}

impl AccountLedger {
    #[instrument]
    pub async fn create(
        &self,
        user_id: Uuid,
        name: &str,
        description: Option<&str>,
    ) -> Result<AccountRecord, Error> {
        let record = query_as!(
            AccountRecord,
            "
            INSERT INTO _ledger.account
                    (user_id, name, description)
                VALUES
                    ($1, $2, $3)
                RETURNING
                    id, created, name, description
            ;
            ",
            user_id,
            name as _,
            description as _
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }
}
