use crate::server::state::AppState;

mod account;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().nest("/account", account::router())
}
