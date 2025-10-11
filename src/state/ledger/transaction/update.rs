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

    #[error("description too long")]
    DescriptionLength,

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
            Some("account_unq") => Self::Duplicate,
            Some("account_chk_name_len") => Self::NameLength,
            Some("account_chk_description_len") => Self::DescriptionLength,

            _ => Self::Database(error),
        }
    }
}

impl TransactionLedger {
    #[instrument]
    pub async fn update(
        &self,
        user_id: Uuid,
        id: Uuid,
        occurred_on: NaiveDate,
        account: Uuid,
        payee: Uuid,
        currency: CurrencyCode,
        amount: f64,
        description: Option<&str>,
    ) -> Result<TransactionRecord, Error> {
        let record = query_as!(
            TransactionRecord,
            "
            UPDATE _ledger.transaction
                SET
                    occurred_on = $3,
                    account = $4,
                    payee = $5,
                    currency = $6,
                    amount = $7,
                    description = $8
                WHERE
                    user_id = $1
                        AND
                    id = $2
                RETURNING
                    id, created, occurred_on, account, payee, currency AS \"currency: CurrencyCode\", amount, description
            ;
            ",
            user_id,
            id,
            occurred_on,
            account,
            payee,
            currency as _,
            amount,
            description as _
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }
}
