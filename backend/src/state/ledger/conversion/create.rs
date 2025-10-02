use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot convert between identical commodities")]
    SameCommodity,

    #[error("conversion already exists")]
    Duplicate,

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        let Some(db_error) = error.as_database_error() else {
            return Self::Database(error);
        };

        match (db_error.code().as_deref(), db_error.constraint()) {
            (Some("23505"), Some("conversion_unq")) => Error::Duplicate,

            _ => Self::Database(error),
        }
    }
}

impl ConversionLedger {
    #[instrument]
    pub async fn create(
        &self,
        user_id: Uuid,
        effective: NaiveDate,
        from: Uuid,
        to: Uuid,
        rate: f64,
    ) -> Result<ConversionRecord, Error> {
        if from == to {
            return Err(Error::SameCommodity);
        }

        let record = query_as!(
            ConversionRecord,
            "
            INSERT INTO _ledger.conversion
                    (user_id, effective, from_commodity, to_commodity, rate)
                VALUES
                    ($1, $2, $3, $4, $5)
                RETURNING
                    id, created, effective, from_commodity, to_commodity, rate
            ;
            ",
            user_id,
            effective,
            from,
            to,
            rate
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }
}
