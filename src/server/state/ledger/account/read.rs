use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not find account")]
    NotFound,

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::NotFound,
            error => Self::Database(error),
        }
    }
}

impl super::AccountLedger {
    #[instrument]
    pub async fn read_all(&self, user_id: Uuid) -> Result<Box<[AccountRecord]>, Error> {
        let accounts = query_as!(
            AccountRecord,
            "
            SELECT id, created, kind AS \"kind: AccountKind\", name, description
                FROM _ledger.account
                WHERE
                    user_id = $1
            ;
            ",
            user_id
        )
        .fetch_all(&self.db)
        .await?
        .into_boxed_slice();

        Ok(accounts)
    }

    #[instrument]
    pub async fn read(&self, user_id: Uuid, id: Uuid) -> Result<AccountRecord, Error> {
        let record = query_as!(
            AccountRecord,
            "
            SELECT id, created, kind AS \"kind: AccountKind\", name, description
                FROM _ledger.account
                WHERE
                    user_id = $1
                        AND
                    id = $2
                LIMIT 1
            ;
            ",
            user_id,
            id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }
}
