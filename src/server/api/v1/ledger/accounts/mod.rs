use crate::server::{
    internal_error,
    state::{get_user_id, AppState},
};
use axum::{
    extract::State,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json,
};
use chrono::{DateTime, Utc};
use tower_sessions::Session;
use uuid::Uuid;

mod create;
mod delete;
mod read;
mod update;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(get_all))
        .route("/", post(create::create))
        .route("/:id", get(read::read))
        .route("/:id", put(update::update))
        .route("/:id", delete(delete::delete))
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, FromPrimitive, Serialize, Deserialize,
)]
#[serde(rename_all = "UPPERCASE")]
#[repr(i16)]
enum Kind {
    #[default]
    Equity = 0,
    Asset = 1,
    Liability = 2,
    Income = 3,
    Expense = 4,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Account {
    id: Uuid,
    created: DateTime<Utc>,
    kind: Kind,
    name: String,
    description: Option<String>,
}

impl IntoResponse for Account {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, Json::from(self)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    kind: Kind,
    name: String,
    description: Option<String>,
}

async fn get_all(session: Session, state: State<AppState>) -> impl IntoResponse {
    query_as!(
        Account,
        "
        SELECT id, created, kind, name, description
            FROM ledger.accounts
            WHERE
                user_id = $1
        ;
        ",
        get_user_id(&session).await
    )
    .fetch_all(state.db())
    .await
    .map_or_else(
        |error| internal_error(error).into_response(),
        |accounts| Json::from(accounts).into_response(),
    )
}
