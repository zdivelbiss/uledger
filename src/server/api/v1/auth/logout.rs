use crate::server::responses::internal_error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower_sessions::Session;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Session(error) => internal_error(error).into_response(),
        }
    }
}

#[axum::debug_handler]
pub async fn logout(session: Session) -> Result<StatusCode, Error> {
    session.flush().await?;

    Ok(StatusCode::OK)
}
