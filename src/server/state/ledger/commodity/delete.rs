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

impl super::AccountLedger {
    #[instrument]
    pub async fn delete(&self, user_id: Uuid, id: Uuid) -> Result<(), Error> {
        let rows_affected = query!(
            "
            DELETE FROM _ledger.account
                WHERE
                    user_id = $2
                        AND
                    id = $1
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
                unreachable!("unexpectedly deleted multiple accounts: {rows_affected} total")
            }
        }
    }
}
