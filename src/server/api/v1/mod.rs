use crate::server::AppState;

mod ledger;
mod user;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/ledger", ledger::router())
        .nest("/user", user::router())
}
