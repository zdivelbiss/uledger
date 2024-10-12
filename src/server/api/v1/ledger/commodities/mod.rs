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

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Commodity {
    id: Uuid,
    created: DateTime<Utc>,
    name: String,
    format: String,
}

impl IntoResponse for Commodity {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, Json::from(self)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommodityInfo {
    name: String,
    format: String,
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
        |commodities| Json::from(commodities).into_response(),
    )
}
