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

impl CommodityLedger {
    #[instrument]
    pub async fn read(&self, user_id: Uuid, id: Uuid) -> Result<CommodityRecord, Error> {
        let record = query_as!(
            CommodityRecord,
            "
            SELECT id, created, name, format
                FROM _ledger.commodity
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
