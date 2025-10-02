use axum::{Router, routing::get};

#[derive(askama::Template)]
#[template(path = "pages/accounts.html")]
struct AccountsTemplate {}

pub fn router() -> Router {
    Router::new().route("/accounts", get(|| async { AccountsTemplate {} }))
}
