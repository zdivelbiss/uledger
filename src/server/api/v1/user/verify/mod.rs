use crate::server::state::AppState;
use axum::routing::post;

mod confirm;
mod create;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/create", post(create::handler))
        .route("/confirm", post(confirm::handler))
}
