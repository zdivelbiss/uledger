use crate::api::state::AppState;
use axum::{
    extract::Request,
    middleware::{from_fn, Next},
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use tower_sessions::Session;
use uuid::Uuid;

mod verify;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/verify", verify::router())
        .layer(from_fn(check_user_session))
}

async fn check_user_session(session: Session, request: Request, next: Next) -> Response {
    match session.get::<Uuid>("user_id").await {
        Ok(Some(_)) => next.run(request).await,

        Ok(_) | Err(_) => (StatusCode::FORBIDDEN, "You must authenticate.").into_response(),
    }
}
