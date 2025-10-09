use crate::state::AppState;

mod account;
mod currency;
mod payee;
mod transaction;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/currency", currency::router())
        .nest("/account", account::router())
        .nest("/payee", payee::router())
        .nest("/transaction", transaction::router())
}
