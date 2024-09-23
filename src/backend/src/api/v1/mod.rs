use crate::api::state::State;

mod auth;
mod ledger;

pub fn routes() -> axum::Router<State> {
    axum::Router::new()
        .nest("/ledger", ledger::routes())
        .nest("/auth", auth::routes())
}
