use crate::{
    api::{
        init_session, internal_error,
        state::{user::UserState, AppState},
    },
    util::{EmailAddress, PasswordDigest},
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Debug, Deserialize)]
pub struct AuthInfo {
    email_address: EmailAddress,
    password_digest: PasswordDigest,
}

#[axum::debug_handler]
pub async fn register(user_state: State<UserState>, body: Json<AuthInfo>) -> impl IntoResponse {
    use crate::api::state::user::register::Error;

    match user_state
        .register(&body.email_address, &body.password_digest)
        .await
    {
        Ok(user_id) => {
            let body_json = serde_json::to_string(&user_id).unwrap();

            (StatusCode::OK, body_json).into_response()
        }

        Err(Error::UserExists) => StatusCode::CONFLICT.into_response(),

        Err(err) => {
            error!("{err:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[axum::debug_handler]
pub async fn login(
    app_state: State<AppState>,
    session: Session,
    headers: HeaderMap,
    body: Json<AuthInfo>,
) -> impl IntoResponse {
    use crate::api::state::user::login::Error;

    match app_state
        .user_state()
        .login(&body.email_address, &body.password_digest)
        .await
    {
        Ok(user_id) => {
            let user_agent = headers.get("User-Agent").and_then(|v| v.to_str().ok());

            match init_session(user_id, user_agent, &session).await {
                Ok(_) => StatusCode::OK.into_response(),

                Err(err) => internal_error(err).into_response(),
            }
        }

        Err(Error::InvalidEmail | Error::InvalidPassword) => {
            StatusCode::UNAUTHORIZED.into_response()
        }

        Err(err) => internal_error(err).into_response(),
    }
}
