use crate::server::App;

mod accounts;
mod commodities;

pub fn router() -> axum::Router<App> {
    axum::Router::new()
    .nest("/accounts", accounts::router())
    .nest("/commodities", commodities::router())
}
