use crate::server::{
    htmx::{hx_redirect, IsHtmx},
    internal_error,
    state::{user::account::UserAccount, App},
};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use lib::EmailAddress;

mod verify;

pub fn router() -> axum::Router<App> {
    axum::Router::new()
        .nest("/verify", verify::router())
        .route("/register", post(_register))
        .route("/login", post(_login))
        .route("/logout", post(_logout))
    // TODO .route("/display_name", get(display_name))
}

#[derive(Debug, Deserialize)]
struct RegisterInfo {
    display_name: String,
    email_address: EmailAddress,
    password: String,
}

async fn _register(
    State(user_account): State<UserAccount>,
    IsHtmx(is_htmx): IsHtmx,
    Form(RegisterInfo {
        display_name,
        email_address,
        password,
    }): Form<RegisterInfo>,
) -> impl IntoResponse {
    use crate::server::state::user::account::register::Error;

    match user_account
        .register(&email_address, password.as_str(), display_name.as_str())
        .await
    {
        Ok(_) if is_htmx => (StatusCode::OK, [hx_redirect("/login")]).into_response(),
        Ok(_) => (StatusCode::OK, "your account has been registered").into_response(),

        Err(Error::DuplicateEmail) => {
            (StatusCode::CONFLICT, "email address already in use").into_response()
        }

        Err(error) => internal_error(error).into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct LoginInfo {
    email_address: EmailAddress,
    password: String,
}

#[allow(clippy::disallowed_types)]
async fn _login(
    session: tower_sessions::Session,
    State(user_account): State<UserAccount>,
    IsHtmx(is_htmx): IsHtmx,
    Form(LoginInfo {
        email_address,
        password,
    }): Form<LoginInfo>,
) -> impl IntoResponse {
    use crate::server::state::user::account::login::Error;

    if !session.is_empty().await {
        return (StatusCode::CONFLICT, "you are already logged in").into_response();
    }

    match user_account.login(&email_address, password.as_str()).await {
        Ok(user_id) => {
            session.insert("id", user_id).await.unwrap();

            if is_htmx {
                (StatusCode::OK, [hx_redirect("/")]).into_response()
            } else {
                (StatusCode::OK, "you have been logged in").into_response()
            }
        }

        Err(Error::InvalidCredentials) => {
            (StatusCode::UNAUTHORIZED, "invalid login credentials").into_response()
        }

        Err(error) => internal_error(error).into_response(),
    }
}

#[allow(clippy::disallowed_types)]
async fn _logout(session: tower_sessions::Session) -> impl IntoResponse {
    if session.is_empty().await {
        (StatusCode::UNAUTHORIZED, "you are not logged in").into_response()
    } else {
        session.flush().await.map_or_else(
            |error| internal_error(error).into_response(),
            |_| (StatusCode::OK, "you have been logged out").into_response(),
        )
    }
}
