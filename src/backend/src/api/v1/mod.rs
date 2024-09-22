use crate::api::State;

mod accounts;

pub fn routes() -> axum::Router<State> {
    axum::Router::new().nest("/accounts", accounts::routes())
}
