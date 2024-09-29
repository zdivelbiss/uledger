use crate::{
    api::app_state::{user_state::UserState, AppState},
    config::cfg,
    util::{EmailAddress, PasswordDigest},
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, Expiration, SameSite},
    PrivateCookieJar,
};
use chrono::Utc;
use serde::Deserialize;

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
    use crate::api::app_state::user_state::register::Error;

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
    private_cookies: PrivateCookieJar,
    headers: HeaderMap,
    body: Json<AuthInfo>,
) -> impl IntoResponse {
    use crate::api::app_state::{session_state::Session, user_state::login::Error};

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
            let session = Session::new(user_id, user_agent);

            let session_id = app_state.session_state().store(session).await.unwrap();

            let mut session_cookie = Cookie::new("id", session_id.to_string());
            session_cookie.set_expires(Expiration::Session);
            session_cookie.set_secure(true);
            session_cookie.set_http_only(true);
            session_cookie.set_same_site(SameSite::Strict);

            let private_cookies = private_cookies.add(session_cookie);

            (StatusCode::OK, private_cookies).into_response()
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
