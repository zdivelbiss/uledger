use crate::server::AppState;
use axum::{routing::get, Router};

#[derive(askama::Template)]
#[template(path = "pages/accounts.html")]
struct AccountsTemplate {}

pub fn router() -> Router<AppState> {
    Router::new().route("/accounts", get(|| async { AccountsTemplate {} }))
}
