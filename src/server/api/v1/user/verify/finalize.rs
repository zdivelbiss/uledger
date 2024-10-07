use crate::{
    server::{get_user_id, responses::internal_error, state::AppState},
    util::{EmailAddress, VerificationToken},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json,
};
use tower_sessions::Session;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/", post(finalize))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not validate verification")]
    NoVerificationMatch,

    #[error("unexpectedly dropped multiple verifications")]
    MultiVerificationDrop,

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        let Some(db_err) = err.as_database_error() else {
            return Self::Database(err);
        };

        match (db_err.code().as_deref(), db_err.constraint()) {
            _ => Self::Database(err),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NoVerificationMatch => {
                (StatusCode::NOT_FOUND, "No matching verification found.").into_response()
            }

            Error::MultiVerificationDrop => {
                internal_error("Dropped multiple verifications!").into_response()
            }
            Error::Database(error) => internal_error(error).into_response(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct RequestBody {
    email_address: EmailAddress,
    proof_token: VerificationToken,
}

#[axum::debug_handler]
async fn finalize(
    session: Session,
    state: State<AppState>,
    body: Json<RequestBody>,
) -> impl IntoResponse {
    let email_address = &body.email_address;
    let proof_token = &body.proof_token;
    let user_id = get_user_id(&session).await;

    let rows_affected = query!(
        "
        DELETE FROM auth.email_verification
            WHERE (user_id, email_address, proof_token) = ($1, $2, $3)
        ;
        ",
        user_id,
        email_address.as_str(),
        proof_token.to_string()
    )
    .execute(state.db())
    .await?
    .rows_affected();

    match rows_affected {
        1 => {
            query!(
                "
                UPDATE auth.users
                    SET
                        email = $2,
                        email_verified_on = $3
                    WHERE id = $1
                ;
                ",
                user_id,
                email_address.as_str(),
                chrono::Utc::now().date_naive()
            )
            .execute(state.db())
            .await?;

            Ok(StatusCode::OK)
        }

        0 => Err(Error::NoVerificationMatch),
        rows_affected => {
            warn!("Dropped {rows_affected} during email validation!");

            Err(Error::MultiVerificationDrop)
        }
    }
}
