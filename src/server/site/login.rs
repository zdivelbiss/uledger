use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tower_sessions::Session;

use crate::server::{internal_error, is_authenticated};

#[derive(askama::Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    is_authenticated: bool,
}

#[axum::debug_handler]
pub async fn serve(session: Session) -> impl axum::response::IntoResponse {
    let is_authenticated = is_authenticated(&session).await;

    match askama::Template::render(&LoginTemplate { is_authenticated }) {
        Ok(render) => (StatusCode::OK, Html::from(render)).into_response(),
        Err(error) => (internal_error(error), "internal server error").into_response(),
    }
}
