use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl PayeeLedger {
    #[instrument]
    pub async fn read_all(&self, user_id: Uuid) -> Result<Box<[PayeeRecord]>, Error> {
        let records = query_as!(
            PayeeRecord,
            "
            SELECT id, created, name, description
                FROM _ledger.payee
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
