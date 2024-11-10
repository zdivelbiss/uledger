use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not find account")]
    NotFound,

    #[error("account name/kind already used")]
    Duplicate,

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

        match (db_error.code().as_deref(), db_error.constraint()) {
            (Some("23505"), Some("accounts_user_id_kind_name_key")) => Error::Duplicate,

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
