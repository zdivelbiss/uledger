use crate::server::{internal_error, state::AppState};
use axum::extract::{Query, State};
use tower_sessions::Session;

use super::{Account, Kind};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal server error")]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        let Some(db_err) = err.as_database_error() else {
            return Self::Database(err);
        };

        match (db_err.code().as_deref(), db_err.constraint()) {
            _ => Self::Database(err),
        }
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(match &self {
                Error::Database(error) => internal_error(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadAccount {
    kind: Kind,
    name: String,
}

pub async fn read(
    session: Session,
    state: State<AppState>,
    params: Query<ReadAccount>,
) -> Result<Account, Error> {
    let user_id = crate::server::state::get_user_id(&session).await;

    query_as!(
        Account,
        "
        SELECT id, created, kind, name, description
            FROM ledger.accounts
                WHERE
                    user_id = $1
                        AND
                    kind = $2
                        AND
                    name = $3
        ;
        ",
        user_id,
        i16::from(params.kind),
        params.name
    )
    .fetch_one(state.db())
    .await
    .map_err(Error::from)
}
