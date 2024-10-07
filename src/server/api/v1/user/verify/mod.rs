use crate::server::state::AppState;

mod create;
mod finalize;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/create", create::router())
        .nest("/finalize", finalize::router())
}
