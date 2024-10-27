use crate::server::{internal_error, AppState, UserSession};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

mod login;
mod register;
mod verify;

pub fn router() -> axum::Router<AppState> {
    // TODO abstract the behaviour of these functions into a UserState object

    axum::Router::new()
        .nest("/verify", verify::router())
        .route("/register", post(register::handler))
        .route("/login", post(login::handler))
        .route("/display_name", get(display_name))
}

async fn display_name(state: State<AppState>, user_session: UserSession) -> impl IntoResponse {
    let user_id = user_session.get_id().await;

    match query!(
        "
        SELECT display_name
            FROM users.profile
            WHERE id = $1
        ;
        ",
        user_id
    )
    .fetch_one(state.db())
    .await
    {
        Ok(record) => (StatusCode::OK, record.display_name).into_response(),
        Err(error) => (internal_error(error), "Internal server error.").into_response(),
    }
}
