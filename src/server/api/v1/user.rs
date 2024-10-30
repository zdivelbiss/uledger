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
                 user_verifiaction: State<UserVerification>,
                 form: Form<Info>| async move {
                    use crate::server::state::user::verification::CreateError;

                    let user_id = user_session.get_user_id().await;
                    match user_verifiaction.create(user_id, &form.email_address).await {
                        Ok(_) => (StatusCode::OK, "verification sent to email").into_response(),

                        Err(CreateError::EmailInUse) => {
                            (StatusCode::CONFLICT, "email already being used").into_response()
                        }
                        Err(CreateError::Database(error)) => internal_error(error).into_response(),
                        Err(CreateError::Postmark(error)) => internal_error(error).into_response(),
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
                 user_verification: State<UserVerification>,
                 form: Form<Info>| async move {
                    use crate::server::state::user::verification::ConfirmError;

                    let user_id = user_session.get_user_id().await;
                    match user_verification
                        .confirm(user_id, &form.email_address, form.proof_token)
                        .await
                    {
                        Ok(_) => (StatusCode::OK, "your email has been verified").into_response(),

                        Err(ConfirmError::NoMatch) => {
                            (StatusCode::NOT_FOUND, "verification does not match").into_response()
                        }
                        Err(ConfirmError::Database(error)) => internal_error(error).into_response(),
                        Err(ConfirmError::Postmark(error)) => internal_error(error).into_response(),
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

                |user_account: State<UserAccount>, is_htmx: IsHtmx, form: Form<Info>| async move {
                    use crate::server::state::user::account::register::Error;

                    match user_account
                        .register(&form.email_address, &form.password, &form.display_name)
                        .await
                    {
                        Ok(_) if *is_htmx => {
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
                |user_account: State<UserAccount>,
                 session: tower_sessions::Session,
                 is_htmx: IsHtmx,
                 form: Form<Info>| async move {
                    use crate::server::state::user::account::login::Error;

                    if !session.is_empty().await {
                        return (StatusCode::CONFLICT, "you are already logged in").into_response();
                    }

                    match user_account
                        .login(&form.email_address, &form.password)
                        .await
                    {
                        Ok(user_id) => {
                            session.insert("id", user_id).await.unwrap();

                            if *is_htmx {
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
    // TODO .route("/display_name", get(display_name))
}
