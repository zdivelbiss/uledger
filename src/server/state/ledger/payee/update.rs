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
            Some("payee_unq") => Self::Duplicate,
            Some("payee_chk_name_len") => Self::NameLength,
            Some("payee_chk_description_len") => Self::DescriptionLength,

            _ => Self::Database(error),
        }
    }
}

impl PayeeLedger {
    #[instrument]
    pub async fn update(
        &self,
        user_id: Uuid,
        id: Uuid,
        name: &str,
        format: &str,
    ) -> Result<PayeeRecord, Error> {
        let record = query_as!(
            PayeeRecord,
            "
            UPDATE _ledger.payee
                SET
                    name = $3,
                    description = $4
                WHERE
                    user_id = $1
                        AND
                    id = $2
                RETURNING
                    id, created, name, description
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
