use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not find account")]
    NotFound,

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl super::AccountLedger {
    #[instrument]
    async fn delete(&self, user_id: Uuid, id: Uuid) -> Result<(), Error> {
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
