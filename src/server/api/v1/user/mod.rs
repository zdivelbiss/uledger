use crate::server::{AppState, UserSession};
use axum::routing::{get, post};

mod login;
mod register;
mod verify;

pub fn router() -> axum::Router<AppState> {
    // TODO abstract the behaviour of these functions into a UserState object

    axum::Router::new()
        .nest("/verify", verify::router())
        .route("/register", post(register::handler))
        .route("/login", post(login::handler))
        .route(
            "/display_name",
            get(|user_session: UserSession| async move { user_session.get_display_name().await }),
        )
}
