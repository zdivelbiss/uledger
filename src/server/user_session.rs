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
    const DISPLAY_NAME: &str = "display_name";

    async fn new(session: Session) -> Option<Self> {
        match (
            session.get_value(Self::USER_ID).await,
            session.get_value(Self::DISPLAY_NAME).await,
        ) {
            (Ok(Some(_)), Ok(Some(_))) => Some(Self(session)),
            _ => None,
        }
    }

    pub async fn get_user_id(&self) -> Uuid {
        self.0.get(Self::USER_ID).await.unwrap().unwrap()
    }

    pub async fn get_display_name(&self) -> String {
        self.0.get(Self::DISPLAY_NAME).await.unwrap().unwrap()
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
