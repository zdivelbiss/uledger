use crate::server::{internal_error, state::AppState};
use axum::{
    extract::{Form, State},
    http::StatusCode,
};
use tower_sessions::Session;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("account already exists")]
    Duplicate,

    #[error("internal server error")]
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

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(match &self {
                Error::Duplicate => StatusCode::CONFLICT,
                Error::Database(error) => internal_error(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateAccount {
    kind: super::Kind,
    name: String,
    description: Option<String>,
}

pub async fn create(
    session: Session,
    state: State<AppState>,
    form: Form<CreateAccount>,
) -> Result<(), Error> {
    let user_id = crate::server::state::get_user_id(&session).await;

    query!(
        "
        INSERT INTO ledger.accounts (user_id, kind, name, description)
            VALUES ($1, $2, $3, $4)
        ;
        ",
        user_id,
        i16::from(form.kind),
        form.name.as_str(),
        form.description.as_deref()
    )
    .execute(state.db())
    .await?;

    Ok(())
}
