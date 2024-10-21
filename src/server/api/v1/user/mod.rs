use crate::server::AppState;
use axum::routing::post;

mod login;
mod register;
mod verify;

pub fn router() -> axum::Router<AppState> {
    // TODO abstract the behaviour of these functions into a UserState object

    axum::Router::new()
        .nest("/verify", verify::router())
        .route("/register", post(register::handler))
        .route("/login", post(login::handler))
}
