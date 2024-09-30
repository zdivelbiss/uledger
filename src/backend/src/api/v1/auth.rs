use crate::{
    api::state::{user::UserState, AppState},
    util::{EmailAddress, PasswordDigest},
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Json,
};
use serde::Deserialize;
use tower_sessions::Session;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

#[derive(Debug, Deserialize)]
struct AuthInfo {
    email_address: EmailAddress,
    password_digest: PasswordDigest,
}

#[axum::debug_handler]
async fn register(user_state: State<UserState>, body: Json<AuthInfo>) -> impl IntoResponse {
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
async fn login(
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
            let user_agent = headers
                .get("User-Agent")
                .and_then(|v| v.to_str().ok())
                .map(str::to_string);

            session.insert("user_id", user_id).await.unwrap();
            session.insert("user_agent", user_agent).await.unwrap();

            StatusCode::OK.into_response()
        }

        Err(Error::InvalidEmail | Error::InvalidPassword) => {
            StatusCode::UNAUTHORIZED.into_response()
        }

        Err(err) => {
            error!("{err:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
