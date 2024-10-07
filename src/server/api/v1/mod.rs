use crate::server::state::AppState;

mod auth;
mod ledger;
mod user;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/ledger", ledger::router())
        .nest("/auth", auth::router())
        .nest("/user", user::router())
}
