use crate::server::state::AppState;

mod accounts;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().nest("/accounts", accounts::router())
}
