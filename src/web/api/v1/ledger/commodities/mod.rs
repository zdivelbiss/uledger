use crate::web::{
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Commodity {
    id: Uuid,
    created: DateTime<Utc>,
    name: String,
    format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommodityInfo {
    name: String,
    format: String,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("internal server error")]
    Database(#[from] sqlx::Error),
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

async fn get_all(session: Session, state: State<AppState>) -> impl IntoResponse {
    query_as!(
        Commodity,
        "
        SELECT id, created, name, format
            FROM ledger.commodities
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
        |commodities| axum::Json::from(commodities).into_response(),
    )
}
