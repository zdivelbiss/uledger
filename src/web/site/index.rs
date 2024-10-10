use axum::{http::StatusCode, response::Html};

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    display_name: String,
}

#[axum::debug_handler]
pub async fn serve() -> impl axum::response::IntoResponse {
    use askama::Template;

    let index_template = IndexTemplate {
        display_name: "John Doe".to_string(),
    };

    let index_rendered = index_template.render().unwrap();
    let index_html = Html::from(index_rendered);

    (StatusCode::OK, index_html)
}
