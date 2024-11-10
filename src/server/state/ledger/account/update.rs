use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("account not found")]
    NotFound,

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
        if let sqlx::Error::RowNotFound = error {
            return Self::NotFound;
        }

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

impl super::AccountLedger {
    #[instrument]
    pub async fn update(
        &self,
        user_id: Uuid,
        id: Uuid,
        kind: AccountKind,
        name: &str,
        description: Option<&str>,
    ) -> Result<AccountRecord, Error> {
        let record = query_as!(
            AccountRecord,
            "
            UPDATE _ledger.account
                SET
                    kind = $3,
                    name = $4,
                    description = $5
                WHERE
                    user_id = $1
                        AND
                    id = $2
                RETURNING
                    id, created, kind AS \"kind: AccountKind\", name, description
            ;
            ",
            user_id,
            id,
            kind as _,
            name as _,
            description as _
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }
}
