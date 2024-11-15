use crate::server::state::App;

mod account;
mod commodity;
mod payee;

pub fn router() -> axum::Router<App> {
    axum::Router::new()
        .nest("/account", account::router())
        .nest("/commodity", commodity::router())
        .nest("/payee", payee::router())
}
