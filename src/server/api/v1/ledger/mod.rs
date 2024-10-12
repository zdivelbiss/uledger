use crate::server::state::AppState;

mod accounts;
mod commodities;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
    .nest("/accounts", accounts::router())
    .nest("/commodities", commodities::router())
}
