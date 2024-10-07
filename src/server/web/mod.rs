use crate::server::state::AppState;
use axum::{http::StatusCode, response::Html, routing::get};
use tower_sessions::Session;

use super::get_user_id;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/", get(serve_index))
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    is_authenticated: bool,
}

#[axum::debug_handler]
async fn serve_index(session: Session) -> impl axum::response::IntoResponse {
    use askama::Template;

    let index_template = IndexTemplate {
        is_authenticated: get_user_id(&session).await.is_some(),
    };
    let index_html = Html::from(index_template.render().unwrap());

    (StatusCode::OK, index_html)
}
