use crate::server::{
    internal_error,
    state::{user::verification::UserVerification, App},
    UserSession,
};
use crate::{EmailAddress, VerificationToken};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
};

pub fn router() -> axum::Router<App> {
    axum::Router::new()
        .route("/", post(_create))
        .route("/", delete(_confirm))
}

#[derive(Debug, Deserialize)]
struct CreateInfo {
    email_address: EmailAddress,
}

#[instrument(skip(user_verification))]
async fn _create(
    user_session: UserSession,
    State(user_verification): State<UserVerification>,
    Form(CreateInfo { email_address }): Form<CreateInfo>,
) -> impl IntoResponse {
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

#[derive(Debug, Deserialize)]
struct ConfirmInfo {
    email_address: EmailAddress,
    proof_token: VerificationToken,
}

#[instrument(skip(user_verification))]
async fn _confirm(
    user_session: UserSession,
    State(user_verification): State<UserVerification>,
    Form(ConfirmInfo {
        email_address,
        proof_token,
    }): Form<ConfirmInfo>,
) -> impl IntoResponse {
    use crate::server::state::user::verification::confirm::Error;

    match user_verification
        .confirm(user_session.id(), &email_address, proof_token)
        .await
    {
        Ok(_) => (StatusCode::OK, "your email has been verified").into_response(),

        Err(Error::NoMatch) => {
            (StatusCode::NOT_FOUND, "verification does not match").into_response()
        }
        Err(error) => internal_error(error).into_response(),
    }
}
