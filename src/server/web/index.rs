use axum::{http::StatusCode, response::Html};
use tower_sessions::Session;

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    is_authenticated: bool,
}

#[axum::debug_handler]
pub async fn serve(session: Session) -> impl axum::response::IntoResponse {
    use askama::Template;

    let index_template = IndexTemplate {
        is_authenticated: false,
    };

    let index_rendered = index_template.render().unwrap();
    let index_html = Html::from(index_rendered);

    (StatusCode::OK, index_html)
}
