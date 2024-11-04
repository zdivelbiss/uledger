use uuid::Uuid;
use super::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("account already exists")]
    Duplicate,

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        let Some(db_err) = err.as_database_error() else {
            return Self::Database(err);
        };

        match (db_err.code().as_deref(), db_err.constraint()) {
            (Some("23505"), Some("accounts_user_id_kind_name_key")) => Error::Duplicate,

            _ => Self::Database(err),
        }
    }
}

impl AccountLedger {
    #[instrument]
    pub async fn create(
        &self,
        user_id: Uuid,
        kind: AccountKind,
        name: &str,
        description: Option<&str>,
    ) -> Result<AccountRecord, Error> {
        let record = query_as!(
            AccountRecord,
            "
            INSERT INTO _ledger.account
                    (user_id, kind, name, description)
                VALUES
                    ($1, $2, $3, $4)
                RETURNING
                    id, created, kind AS \"kind: AccountKind\", name, description
            ;
            ",
            user_id,
            kind as _,
            name as _,
            description as _
        )
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }
}
