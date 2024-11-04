use crate::{
    server::{
        htmx::{hx_redirect, IsHtmx},
        internal_error,
        state::{
            user::{profile::UserProfile, verification::UserVerification},
            App,
        },
        UserSession,
    },
    VerificationToken,
};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};

pub fn router() -> axum::Router<App> {
    axum::Router::new()
        .route(
            "/register",
            post({
                #[derive(Debug, Deserialize)]
                struct Info {
                    display_name: String,
                    email_address: String,
                    password: String,
                }

                |State(user_profile): State<UserProfile>,
                 IsHtmx(is_htmx): IsHtmx,
                 Form(Info {
                     display_name,
                     email_address,
                     password,
                 }): Form<Info>| async move {
                    use crate::server::state::user::profile::register::Error;

                    match user_profile
                        .register(
                            email_address.as_str(),
                            password.as_str(),
                            display_name.as_str(),
                        )
                        .await
                    {
                        Ok(_) if is_htmx => {
                            (StatusCode::OK, [hx_redirect("/login")]).into_response()
                        }
                        Ok(_) => {
                            (StatusCode::OK, "your account has been registered").into_response()
                        }

                        Err(Error::DuplicateEmail) => {
                            (StatusCode::CONFLICT, "email address already in use").into_response()
                        }

                        Err(error) => internal_error(error).into_response(),
                    }
                }
            }),
        )
        .route(
            "/login",
            post({
                #[derive(Debug, Deserialize)]
                struct Info {
                    email_address: String,
                    password: String,
                }

                #[allow(clippy::disallowed_types)]
                |session: tower_sessions::Session,
                 State(user_profile): State<UserProfile>,
                 IsHtmx(is_htmx): IsHtmx,
                 Form(Info {
                     email_address,
                     password,
                 }): Form<Info>| async move {
                    use crate::server::state::user::profile::login::Error;

                    if !session.is_empty().await {
                        return (StatusCode::CONFLICT, "you are already logged in").into_response();
                    }

                    match user_profile.login(&email_address, password.as_str()).await {
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
            }),
        )
        .route(
            "/logout",
            post({
                #[allow(clippy::disallowed_types)]
                |session: tower_sessions::Session| async move {
                    if session.is_empty().await {
                        (StatusCode::UNAUTHORIZED, "you are not logged in").into_response()
                    } else {
                        session.flush().await.map_or_else(
                            |error| internal_error(error).into_response(),
                            |_| (StatusCode::OK, "you have been logged out").into_response(),
                        )
                    }
                }
            }),
        )
        .route(
            "/verify",
            post({
                #[derive(Debug, Deserialize)]
                struct Info {
                    verification_token: String,
                }

                |user_session: UserSession,
                 State(user_verification): State<UserVerification>,
                 Form(Info { verification_token }): Form<Info>| async move {
                    use crate::server::state::user::verification::confirm::Error;

                    let Ok(verification_token) = VerificationToken::from_str(verification_token)
                    else {
                        return (StatusCode::BAD_REQUEST, "malformed verification token")
                            .into_response();
                    };

                    match user_verification
                        .confirm(user_session.id(), verification_token)
                        .await
                    {
                        Ok(_) => (StatusCode::OK, "your email has been verified").into_response(),

                        Err(Error::NoMatch) => {
                            (StatusCode::NOT_FOUND, "verification does not match").into_response()
                        }
                        Err(error) => internal_error(error).into_response(),
                    }
                }
            })
            .delete({
                #[derive(Debug, Deserialize)]
                struct Info {
                    email_address: String,
                }
                |user_session: UserSession,
                 State(user_verification): State<UserVerification>,
                 Form(Info { email_address }): Form<Info>| async move {
                    use crate::server::state::user::verification::create::Error;

                    match user_verification
                        .create(user_session.id(), &email_address)
                        .await
                    {
                        Ok(_) => (StatusCode::OK, "verification sent to email").into_response(),

                        Err(Error::EmailInUse) => {
                            (StatusCode::CONFLICT, "email already being used").into_response()
                        }

                        Err(error) => internal_error(error).into_response(),
                    }
                }
            }),
        )
    // TODO .route("/display_name", get(display_name))
}
