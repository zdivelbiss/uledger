use crate::state::AppState;

mod account;
mod commodity;
mod payee;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/account", account::router())
        .nest("/commodity", commodity::router())
        .nest("/payee", payee::router())
}
