use crate::server::state::App;

mod account;
mod commodity;

pub fn router() -> axum::Router<App> {
    axum::Router::new()
    .nest("/account", account::router())
    .nest("/commodity", commodity::router())
}
