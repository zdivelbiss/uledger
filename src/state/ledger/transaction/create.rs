use super::*;
use crate::util::CurrencyCode;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("currency code length is not 3")]
    CurrencyCodeLength,

    #[error("currency code is not supported")]
    CurrencyCodeSupport,

    #[error("description too long")]
    DescriptionLength,

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        let Some(db_error) = error.as_database_error() else {
            return Self::Database(error);
        };

        match db_error.constraint() {
            Some("chk_currency_code_len") => Self::CurrencyCodeLength,
            Some("chk_currency_code_is_iso") => Self::CurrencyCodeSupport,
            Some("transaction_chk_description_len") => Self::DescriptionLength,

            _ => Self::Database(error),
        }
    }
}

impl TransactionLedger {
    #[instrument]
    pub async fn create(
        &self,
        user_id: Uuid,
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
            INSERT INTO _ledger.transaction
                    (user_id, occurred_on, account, payee, currency, amount, description)
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7)
                RETURNING
                    id, created, occurred_on, account, payee, currency AS \"currency: CurrencyCode\", amount, description
            ;
            ",
            user_id,
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
