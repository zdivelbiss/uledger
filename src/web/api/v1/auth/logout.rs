use crate::web::{
    htmx::{hx_redirect, is_htmx},
    internal_error,
};
use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use tower_sessions::Session;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal server error")]
    Session(#[from] tower_sessions::session::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(match &self {
                Error::Session(error) => internal_error(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

#[axum::debug_handler]
pub async fn logout(session: Session, headers: HeaderMap) -> Result<impl IntoResponse, Error> {
    session.flush().await?;

    Ok(if is_htmx(&headers) {
        [hx_redirect("/login")].into_response()
    } else {
        ().into_response()
    })
}
