use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("account already exists")]
    Duplicate,

    #[error("account name too long")]
    NameLength,

    #[error("account description too long")]
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
            Some("account_unq") => Error::Duplicate,
            Some("account_chk_name_len") => Error::NameLength,
            Some("account_chk_description_len") => Error::DescriptionLength,

            _ => Self::Database(error),
        }
    }
}

impl AccountLedger {
    #[instrument]
    pub async fn create(
        &self,
        user_id: Uuid,
        kind: AccountKind,
        name: &str,
        description: Option<&str>,
    ) -> Result<AccountRecord, Error> {
        let record = query_as!(
            AccountRecord,
            "
            INSERT INTO _ledger.account
                    (user_id, kind, name, description)
                VALUES
                    ($1, $2, $3, $4)
                RETURNING
                    id, created, kind AS \"kind: AccountKind\", name, description
            ;
            ",
            user_id,
            kind as _,
            name as _,
            description as _
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }
}
