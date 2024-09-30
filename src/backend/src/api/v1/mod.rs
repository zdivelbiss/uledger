use crate::api::state::AppState;

mod auth;
mod ledger;
mod user;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/ledger", ledger::routes())
        .nest("/auth", auth::routes())
        .nest("/user", user::routes())
}
