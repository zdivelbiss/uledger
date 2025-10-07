#![allow(clippy::disallowed_types)]

use super::internal_error;
use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::{StatusCode, request::Parts},
};
use tower_sessions::Session;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("session is missing data for user")]
    MissingData,

    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),
}

#[derive(Debug)]
pub struct UserSession {
    session: Session,
    id: Uuid,
}

impl UserSession {
    async fn read(session: Session) -> Result<Self, Error> {
        let id = session.get("id").await?.ok_or(Error::MissingData)?;

        Ok(Self { session, id })
    }

    pub const fn id(&self) -> Uuid {
        self.id
    }

    pub async fn logout(self) {
        if let Err(error) = self.session.delete().await {
            warn!("Failed to delete session: {error:?}");
        }
    }
}

impl<S: Sync + Send> FromRequestParts<S> for UserSession {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;

        UserSession::read(session)
            .await
            .map_err(|error| match error {
                Error::MissingData => (StatusCode::UNAUTHORIZED, "You are not authorized."),
                error => internal_error(error),
            })
    }
}

impl<S: Sync + Send> OptionalFromRequestParts<S> for UserSession {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;

        match UserSession::read(session).await {
            Ok(user_session) => Ok(Some(user_session)),
            Err(Error::MissingData) => Ok(None),
            Err(error) => Err(internal_error(error)),
        }
    }
}
