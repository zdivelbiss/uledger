use crate::server::state::AppState;
use axum::routing::{delete, get, patch, post};
use chrono::{DateTime, Utc};
use uuid::Uuid;

mod create;
mod delete;
mod read;
mod update;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", post(create::create))
        .route("/", get(read::read))
    // .route("/", patch(update::update))
    // .route("/", delete(delete::delete))
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

impl axum::response::IntoResponse for Account {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json::from(self)).into_response()
    }
}
