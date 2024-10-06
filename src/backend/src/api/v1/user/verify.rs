use crate::{
    api::{
        get_user_id, internal_error,
        state::{user::UserState, AppState},
    },
    util::{EmailAddress, VerificationToken},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json};
use tower_sessions::Session;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/", post(begin_verify).patch(finalize_verify))
}

#[derive(Debug, serde::Deserialize)]
struct BeginVerify {
    email_address: EmailAddress,
}

#[axum::debug_handler]
async fn begin_verify(
    session: Session,
    user_state: State<UserState>,
    body: Json<BeginVerify>,
) -> impl IntoResponse {
    use crate::{
        api::state::user::verify::create::Error,
        postmark::{send_email, Transaction},
    };

    let user_id = get_user_id(&session).await.unwrap();

    match user_state
        .create_email_verification(user_id, &body.email_address)
        .await
    {
        Ok(token) => {
            let transaction =
                Transaction::verification(&body.email_address, chrono::Utc::now(), token);

            match send_email(transaction).await {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(err) => internal_error(err).into_response(),
            }
        }

        Err(Error::UserNotExists) => StatusCode::NOT_FOUND.into_response(),
        Err(Error::EmailInUse) => StatusCode::CONFLICT.into_response(),

        Err(err) => internal_error(err).into_response(),
    }
}

#[derive(Debug, serde::Deserialize)]
struct FinalizeVerify {
    email_address: EmailAddress,
    proof_token: VerificationToken,
}

#[axum::debug_handler]
async fn finalize_verify(
    session: Session,
    user_state: State<UserState>,
    body: Json<FinalizeVerify>,
) -> impl IntoResponse {
    let user_id = get_user_id(&session).await.unwrap();

    match user_state
        .finalize_email_verification(user_id, &body.email_address, &body.proof_token)
        .await
    {
        Ok(true) => StatusCode::CREATED.into_response(),

        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => internal_error(err).into_response(),
    }
}
