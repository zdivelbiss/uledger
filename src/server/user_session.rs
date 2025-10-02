#![allow(clippy::disallowed_types)]

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use tower_sessions::Session;
use uuid::Uuid;
use super::internal_error;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("session is missing data for user")]
    MissingData,

    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),
}

#[derive(Debug)]
pub struct UserSession {
    id: Uuid,
}

impl UserSession {
    async fn read(session: &Session) -> Result<Self, Error> {
        let id = session.get("id").await?.ok_or(Error::MissingData)?;

        Ok(Self { id })
    }

    pub const fn id(&self) -> Uuid {
        self.id
    }
}

impl<S: Sync + Send> FromRequestParts<S> for UserSession {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;

        UserSession::read(&session)
            .await
            .map_err(|error| match error {
                Error::MissingData => (StatusCode::UNAUTHORIZED, "You are not authorized."),
                error => internal_error(error),
            })
    }
}
