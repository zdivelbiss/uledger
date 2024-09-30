use crate::api::state::AppState;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

mod auth;
mod verify;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/test", get(test))
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .nest("/verify", verify::routes())
}

#[axum::debug_handler]
async fn test() -> impl IntoResponse {
    (StatusCode::OK, "Test successful.")
}
