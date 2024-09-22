use crate::api::State;
use axum::{http::StatusCode, response::IntoResponse, routing::get};

pub fn routes() -> axum::Router<State> {
    axum::Router::new().route("/test", get(test))
}

async fn test() -> impl IntoResponse {
    (StatusCode::OK, "It worked!")
}
