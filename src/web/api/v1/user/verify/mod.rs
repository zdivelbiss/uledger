use crate::web::state::AppState;

mod confirm;
mod create;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/create", create::router())
        .nest("/confirm", confirm::router())
}
