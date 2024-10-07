use crate::server::state::AppState;

mod verify;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().nest("/verify", verify::router())
}
