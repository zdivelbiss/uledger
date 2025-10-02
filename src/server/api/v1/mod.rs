use crate::state::App;

mod ledger;
mod user;

pub fn router() -> axum::Router<App> {
    axum::Router::new()
        .nest("/ledger", ledger::router())
        .nest("/user", user::router())
}
