use crate::api::app_state::AppState;

mod accounts;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new().nest("/accounts", accounts::routes())
}
