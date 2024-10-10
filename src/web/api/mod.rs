use crate::web::state::AppState;

mod v1;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().nest("/v1", v1::router())
}
