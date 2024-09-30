use crate::api::state::AppState;
use axum::{http::StatusCode, response::IntoResponse, routing::get};

mod verify;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/:user_id/verify", verify::routes())
        .route("/:user_id/test", get(test))
}

#[axum::debug_handler]
async fn test() -> impl IntoResponse {
    (StatusCode::OK, "Test successful.")
}
