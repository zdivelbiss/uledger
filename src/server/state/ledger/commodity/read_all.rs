use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl CommodityLedger {
    #[instrument]
    pub async fn read_all(&self, user_id: Uuid) -> Result<Box<[CommodityRecord]>, Error> {
        let accounts = query_as!(
            CommodityRecord,
            "
            SELECT id,
                   created,
                   name,
                   description,
                   symbol,
                   thousands_separator,
                   decimal_separator,
                   is_prefix
                FROM _ledger.commodity
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
