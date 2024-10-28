use crate::server::state::App;

mod v1;

pub fn router() -> axum::Router<App> {
    axum::Router::new().nest("/v1", v1::router())
}
