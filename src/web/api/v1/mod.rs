use axum::middleware::from_fn;

use crate::web::{authentication_layer, state::AppState};

mod auth;
mod ledger;
mod user;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/ledger", ledger::router())
        .nest("/user", user::router())
        .layer(from_fn(authentication_layer))
        .nest("/auth", auth::router())
}
