#![allow(clippy::disallowed_types)]

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use tower_sessions::Session;
use uuid::Uuid;

pub struct UserSession(Session);

impl UserSession {
    const USER_ID: &str = "user_id";

    async fn new(session: Session) -> Option<Self> {
        if let Ok(Some(_)) = session.get_value(Self::USER_ID).await {
            Some(Self(session))
        } else {
            None
        }
    }

    pub async fn get_user_id(&self) -> Uuid {
        self.0.get(Self::USER_ID).await.unwrap().unwrap()
    }

    pub async fn destroy(&self) {
        if let Err(error) = self.0.flush().await {
            error!("{error:?}");
        }
    }
}

#[axum::async_trait]
impl<S: Sync + Send> FromRequestParts<S> for UserSession {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;

        UserSession::new(session)
            .await
            .ok_or((StatusCode::UNAUTHORIZED, "You are not authorized."))
    }
}
