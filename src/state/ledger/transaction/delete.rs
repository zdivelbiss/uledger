use super::*;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("transaction was not found")]
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

impl TransactionLedger {
    #[instrument]
    pub async fn delete(&self, user_id: Uuid, id: Uuid) -> Result<(), Error> {
        let rows_affected = query!(
            "
            DELETE FROM _ledger.transaction
                WHERE
                    user_id = $1
                        AND
                    id = $2
            ;
            ",
            user_id,
            id
        )
        .execute(&self.db)
        .await?
        .rows_affected();

        match rows_affected {
            1 => Ok(()),

            0 => Err(Error::NotFound),

            rows_affected => {
                unreachable!("deleted multiple: {rows_affected} total")
            }
        }
    }
}
