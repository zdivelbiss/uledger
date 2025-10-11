use super::*;
use uuid::Uuid;
use crate::util::CurrencyCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl TransactionLedger {
    #[instrument]
    pub async fn read_all(&self, user_id: Uuid) -> Result<Box<[TransactionRecord]>, Error> {
        let records = query_as!(
            TransactionRecord,
            "
            SELECT id, created, occurred_on, account, payee, currency AS \"currency: CurrencyCode\", amount, description
                FROM _ledger.transaction
                WHERE
                    user_id = $1
            ;
            ",
            user_id
        )
        .fetch_all(&self.db)
        .await?
        .into_boxed_slice();

        Ok(records)
    }
}
