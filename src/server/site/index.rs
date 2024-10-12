#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    display_name: String,
}

#[axum::debug_handler]
pub async fn serve() -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::OK,
        IndexTemplate {
            display_name: "John Doe".to_string(),
        },
    )
}
