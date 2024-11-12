use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("already exists")]
    Duplicate,

    #[error("name too long")]
    NameLength,

    #[error("description too long")]
    DescriptionLength,

    #[error("symbol too long")]
    SymbolLength,

    #[error("thousands separator too long")]
    ThousandsSeparatorLength,

    #[error("decimal separator too long")]
    DecimalSeparatorLength,

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
            Some("chk_commodity_description_len") => Self::DescriptionLength,
            Some("chk_commodity_symbol_len") => Self::SymbolLength,
            Some("chk_commodity_thousands_separator_len") => Self::ThousandsSeparatorLength,
            Some("chk_commodity_decimal_separator_len") => Self::DecimalSeparatorLength,

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
        description: Option<&str>,
        symbol: &str,
        thousands_separator: &str,
        decimal_separator: &str,
        is_prefix: bool,
    ) -> Result<CommodityRecord, Error> {
        let record = query_as!(
            CommodityRecord,
            "
            INSERT INTO _ledger.commodity
                    (user_id,
                     name,
                     description,
                     symbol,
                     thousands_separator,
                     decimal_separator,
                     is_prefix)
                VALUES
                    ($1, $2, $3,  $4, $5, $6, $7)
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
