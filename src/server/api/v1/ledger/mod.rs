use crate::state::AppState;

mod account;
mod currency;
mod payee;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/account", account::router())
        .nest("/currency", currency::router())
        .nest("/payee", payee::router())
}
