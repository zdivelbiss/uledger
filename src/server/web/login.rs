use axum::{http::StatusCode, response::Html};
use tower_sessions::Session;

use crate::server::is_authenticated;

#[derive(askama::Template)]
#[template(path = "login/page.html")]
struct LoginTemplate {
    is_authenticated: bool,
}

#[axum::debug_handler]
pub async fn serve(session: Session) -> impl axum::response::IntoResponse {
    let is_authenticated = is_authenticated(&session).await;
    let login_render = askama::Template::render(&LoginTemplate { is_authenticated }).unwrap();

    (StatusCode::OK, Html::from(login_render))
}
