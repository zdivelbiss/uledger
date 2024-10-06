use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower_sessions::Session;

use crate::api::internal_error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown logout failure")]
    FlushFail(#[from] tower_sessions::session::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(match self {
                Error::FlushFail(error) => internal_error(error),
            })
            .body(Body::empty())
            .unwrap()
    }
}

#[axum::debug_handler]
pub async fn logout(session: Session) -> Result<StatusCode, Error> {
    session.flush().await?;

    Ok(StatusCode::OK)
}
