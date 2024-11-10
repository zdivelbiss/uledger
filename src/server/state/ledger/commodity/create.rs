use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("already exists")]
    Duplicate,

    #[error("name too long")]
    NameLength,

    #[error("format is too long")]
    FormatLength,

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        let Some(db_error) = error.as_database_error() else {
            return Self::Database(error);
        };

        match db_error.constraint() {
            Some("commodity_unq") => Self::Duplicate,
            Some("commodity_chk_name_len") => Self::NameLength,
            Some("commodity_chk_format_len") => Self::FormatLength,

            _ => Self::Database(error),
        }
    }
}

impl CommodityLedger {
    #[instrument]
    pub async fn create(
        &self,
        user_id: Uuid,
        name: &str,
        format: &str,
    ) -> Result<CommodityRecord, Error> {
        let record = query_as!(
            CommodityRecord,
            "
            INSERT INTO _ledger.commodity
                    (user_id, name, format)
                VALUES
                    ($1, $2, $3)
                RETURNING
                    id, created, name, format
            ;
            ",
            user_id,
            name as _,
            format as _
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }
}
