use axum::{http::StatusCode, response::IntoResponse};
use tower_sessions::Session;

use crate::server::internal_error;

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
pub async fn logout(session: Session) -> Result<StatusCode, Error> {
    session.flush().await?;

    Ok(StatusCode::OK)
}
