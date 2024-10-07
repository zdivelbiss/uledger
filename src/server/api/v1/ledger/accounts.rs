use crate::server::state::AppState;
use axum::{http::StatusCode, response::IntoResponse, routing::get};

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/test", get(test))
}

async fn test() -> impl IntoResponse {
    (StatusCode::OK, "It worked!")
}
