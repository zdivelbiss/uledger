use crate::api::app_state::AppState;
use axum::{http::StatusCode, response::IntoResponse, routing::get};

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new().route("/test", get(test))
}

async fn test() -> impl IntoResponse {
    (StatusCode::OK, "It worked!")
}
