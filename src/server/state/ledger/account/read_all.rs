use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
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
}
