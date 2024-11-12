use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not find")]
    NotFound,

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
        if let sqlx::Error::RowNotFound = error {
            return Self::NotFound;
        }

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
    pub async fn update(
        &self,
        user_id: Uuid,
        id: Uuid,
        name: &str,
        description: Option<&str>,
        symbol: &str,
        thousands_separator: &str,
        decimal_separator: &str,
        is_prefix: bool,
    ) -> Result<CommodityRecord, Error> {
        let record = query_as!(
            CommodityRecord,
            "
            UPDATE _ledger.commodity
                SET
                    name = $3,
                    description = $4,
                    symbol = $5,
                    thousands_separator = $6,
                    decimal_separator = $7,
                    is_prefix = $8
                WHERE
                    user_id = $1
                        AND
                    id = $2
                RETURNING
                    id,
                    created,
                    name,
                    description,
                    symbol,
                    thousands_separator,
                    decimal_separator,
                    is_prefix
            ;
            ",
            user_id,
            id,
            name as _,
            description as _,
            symbol as _,
            thousands_separator as _,
            decimal_separator as _,
            is_prefix
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }
}
