use crate::{
    server::{
        UserSession,
        htmx::{IsHtmx, hx_redirect},
        internal_error,
    },
    state::{
        AppState,
        user::{profile::UserProfile, verification::UserVerification},
    },
    util::VerificationToken,
};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
    routing,
};

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", routing::post(_register))
        .route("/login", routing::post(_login))
        .route("/logout", routing::post(_logout))
        .route("/verify", routing::post(_verify).delete(_delete))
        .route("/display_name", routing::get(_display_name))
}

#[derive(Debug, Deserialize)]
struct RegisterInfo {
    display_name: String,
    email_address: String,
    password: String,
}

async fn _register(
    State(user_profile): State<UserProfile>,
    Form(RegisterInfo {
        display_name,
        email_address,
        password,
    }): Form<RegisterInfo>,
) -> impl IntoResponse {
    use crate::state::user::profile::register::Error;

    match user_profile
        .register(
            email_address.as_str(),
            password.as_str(),
            display_name.as_str(),
        )
        .await
    {
        Ok(_) => (StatusCode::OK, "Your account has been registered.").into_response(),

        Err(Error::EmailAddressLength) => {
            (StatusCode::BAD_REQUEST, "Email address is too long.").into_response()
        }

        Err(Error::EmailAddressFormat) => {
            (StatusCode::BAD_REQUEST, "Email address is invalid.").into_response()
        }

        Err(Error::DisplayNameLength) => {
            (StatusCode::BAD_REQUEST, "Display name is too long.").into_response()
        }

        Err(Error::DuplicateEmail) => {
            (StatusCode::CONFLICT, "Email address is already in use.").into_response()
        }

        Err(error) => internal_error(error).into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct LoginInfo {
    email_address: String,
    password: String,
}

#[allow(clippy::disallowed_types)]
async fn _login(
    session: tower_sessions::Session,
    IsHtmx(is_htmx): IsHtmx,
    State(user_profile): State<UserProfile>,
    Form(LoginInfo {
        email_address,
        password,
    }): Form<LoginInfo>,
) -> impl IntoResponse {
    use crate::state::user::profile::login::Error;

    if !session.is_empty().await {
        return (StatusCode::CONFLICT, "You are already logged in.").into_response();
    }

    match user_profile.login(&email_address, password.as_str()).await {
        Ok(user_id) => {
            session.insert("id", user_id).await.unwrap();

            if is_htmx {
                (StatusCode::OK, [hx_redirect("/")]).into_response()
            } else {
                (StatusCode::OK, "You have been logged in.").into_response()
            }
        }

        Err(Error::InvalidCredentials) => {
            (StatusCode::UNAUTHORIZED, "Wrong username or password.").into_response()
        }

        Err(error) => internal_error(error).into_response(),
    }
}

#[allow(clippy::disallowed_types)]
async fn _logout(session: tower_sessions::Session, IsHtmx(is_htmx): IsHtmx) -> impl IntoResponse {
    if session.is_empty().await {
        (StatusCode::UNAUTHORIZED, "You are not logged in.").into_response()
    } else {
        session.flush().await.map_or_else(
            |error| internal_error(error).into_response(),
            |_| {
                if is_htmx {
                    (StatusCode::OK, [hx_redirect("/login")]).into_response()
                } else {
                    (StatusCode::OK, "You have been logged out.").into_response()
                }
            },
        )
    }
}

#[derive(Debug, Deserialize)]
struct VerifyInfo {
    verification_token: String,
}

async fn _verify(
    user_session: UserSession,
    State(user_verification): State<UserVerification>,
    Form(VerifyInfo { verification_token }): Form<VerifyInfo>,
) -> impl IntoResponse {
    use crate::state::user::verification::confirm::Error;

    let Ok(verification_token) = VerificationToken::from_str(verification_token) else {
        return (StatusCode::BAD_REQUEST, "Verification code is malformed.").into_response();
    };

    match user_verification
        .confirm(user_session.id(), verification_token)
        .await
    {
        Ok(_) => (StatusCode::OK, "Your email has been verified.").into_response(),

        Err(Error::NoMatch) => {
            (StatusCode::NOT_FOUND, "Verification code does not match.").into_response()
        }
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _delete(
    user_session: UserSession,
    State(user_verification): State<UserVerification>,
    Form(email_address): Form<String>,
) -> impl IntoResponse {
    use crate::state::user::verification::create::Error;

    match user_verification
        .create(user_session.id(), &email_address)
        .await
    {
        Ok(_) => (StatusCode::OK, "Verification sent to email.").into_response(),

        Err(Error::EmailInUse) => {
            (StatusCode::CONFLICT, "Email already being used.").into_response()
        }

        Err(error) => internal_error(error).into_response(),
    }
}

async fn _display_name(
    user_session: UserSession,
    State(user_profile): State<UserProfile>,
) -> impl IntoResponse {
    match user_profile.get_display_name(&user_session).await {
        Ok(display_name) => (StatusCode::OK, display_name).into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}
