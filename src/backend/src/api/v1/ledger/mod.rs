use crate::api::state::AppState;

mod accounts;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().nest("/accounts", accounts::router())
}
