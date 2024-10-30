use lib::ledger::account::{AccountKind, AccountRecord};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not find account")]
    NotFound,

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl super::AccountLedger {
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
