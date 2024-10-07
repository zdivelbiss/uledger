use crate::server::{get_user_id, responses::internal_error, state::AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use uuid::Uuid;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(get_all))
        .route("/:account_id", get(get_one))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Database(error) => internal_error(error).into_response(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Kind {
    Equity,
    Asset,
    Liability,
    Income,
    Expense,
}

impl From<String> for Kind {
    fn from(value: String) -> Self {
        info!("{}", value);

        serde_json::from_str(&value).expect("invalid")
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Account {
    id: Uuid,
    user_id: Uuid,
    created: DateTime<Utc>,
    kind: Kind,
    name: String,
    description: Option<String>,
}

async fn get_all(session: Session, state: State<AppState>) -> Result<Json<Vec<Account>>, Error> {
    let user_id = get_user_id(&session).await;

    let accounts = query_as!(
        Account,
        "
        SELECT * FROM ledger.accounts
            WHERE user_id = $1
        ;
        ",
        user_id
    )
    .fetch_all(state.db())
    .await?;

    Ok(accounts.into())
}

async fn get_one() -> Result<Json<Account>, Error> {
    todo!()
}
