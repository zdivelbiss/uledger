use lib::ledger::account::AccountKind;

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

pub struct CreateAccount {
    kind: AccountKind,
    name: String,
    description: Option<String>,
}

impl crate::server::crud::Create for CreateAccount {
    type Args = uuid::Uuid;
    type Error = Error;

    async fn create(self, db: &crate::Datastore, user_id: Self::Args) -> Result<(), Self::Error> {
        query!(
            "
            INSERT INTO _ledger.account
                    (user_id, kind, name, description)
                VALUES
                    ($1, $2, $3, $4)
            ;
            ",
            user_id,
            self.kind as _,
            self.name.as_str(),
            self.description.as_deref()
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
