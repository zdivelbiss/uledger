use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not find commodity")]
    NotFound,

    #[error("commodity already exists")]
    Duplicate,

    #[error("commodity name is too long")]
    NameLength,

    #[error("commodity format is too long")]
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
            Some("commodity_unq") => Error::Duplicate,
            Some("commodity_chk_name_len") => Error::NameLength,
            Some("commodity_chk_format_len") => Error::FormatLength,

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
        format: &str,
    ) -> Result<CommodityRecord, Error> {
        let record = query_as!(
            CommodityRecord,
            "
            UPDATE _ledger.commodity
                SET
                    name = $3,
                    format = $4
                WHERE
                    user_id = $1
                        AND
                    id = $2
                RETURNING
                    id, created, name, format
            ;
            ",
            user_id,
            id,
            name as _,
            format as _
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }
}
