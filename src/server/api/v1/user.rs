use crate::server::{
    htmx::{hx_redirect, IsHtmx},
    internal_error,
    state::{
        user::{account::UserAccount, verification::UserVerification},
        App,
    },
    UserSession,
};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use lib::{EmailAddress, VerificationToken};

pub fn router() -> axum::Router<App> {
    axum::Router::new()
        .route(
            "/verify",
            post({
                #[derive(Debug, Deserialize)]
                struct Info {
                    email_address: EmailAddress,
                }

                |user_session: UserSession,
                 State(user_verifiaction): State<UserVerification>,
                 Form(Info { email_address }): Form<Info>| async move {
                    use crate::server::state::user::verification::create::Error;

                    let user_id = user_session.get_user_id().await;
                    match user_verifiaction.create(user_id, &email_address).await {
                        Ok(_) => (StatusCode::OK, "verification sent to email").into_response(),

                        Err(Error::EmailInUse) => {
                            (StatusCode::CONFLICT, "email already being used").into_response()
                        }

                        Err(error) => internal_error(error).into_response(),
                    }
                }
            })
            .delete({
                #[derive(Debug, Deserialize)]
                struct Info {
                    email_address: EmailAddress,
                    proof_token: VerificationToken,
                }

                |user_session: UserSession,
                 State(user_verification): State<UserVerification>,
                 Form(Info {
                     email_address,
                     proof_token,
                 }): Form<Info>| async move {
                    use crate::server::state::user::verification::confirm::Error;

                    let user_id = user_session.get_user_id().await;
                    match user_verification
                        .confirm(user_id, &email_address, proof_token)
                        .await
                    {
                        Ok(_) => (StatusCode::OK, "your email has been verified").into_response(),

                        Err(Error::NoMatch) => {
                            (StatusCode::NOT_FOUND, "verification does not match").into_response()
                        }
                        Err(error) => internal_error(error).into_response(),
                    }
                }
            }),
        )
        .route(
            "/register",
            post({
                #[derive(Debug, Deserialize)]
                struct Info {
                    display_name: String,
                    email_address: EmailAddress,
                    password: String,
                }

                |State(user_account): State<UserAccount>,
                 IsHtmx(is_htmx): IsHtmx,
                 Form(Info {
                     display_name,
                     email_address,
                     password,
                 }): Form<Info>| async move {
                    use crate::server::state::user::account::register::Error;

                    match user_account
                        .register(&email_address, password.as_str(), display_name.as_str())
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
                    email_address: EmailAddress,
                    password: String,
                }

                #[allow(clippy::disallowed_types)]
                |session: tower_sessions::Session,
                 State(user_account): State<UserAccount>,
                 IsHtmx(is_htmx): IsHtmx,
                 Form(Info {
                     email_address,
                     password,
                 }): Form<Info>| async move {
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
            }),
        )
        .route(
            "/logout",
            post({
                #[allow(clippy::disallowed_types)]
                |user_session: UserSession| async move {
                    user_session.destroy().await;

                    axum::response::Redirect::temporary("/login")
                }
            }),
        )
    // TODO .route("/display_name", get(display_name))
}
