use crate::{
    postmark,
    server::{
        internal_error, internal_error_old,
        state::{user::UserVerification, App},
        UserSession,
    },
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{post, delete}, Router,
};
use lib::EmailAddress;

pub fn router() -> Router<App> {
    Router::new()
        .route("/", post(create))
        .route("/", delete(confirm))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("email already in use")]
    EmailInUse,

    #[error("no verification match")]
    NoVerificationMatch,

    #[error(transparent)]
    Database(sqlx::Error),

    #[error(transparent)]
    Postmark(#[from] crate::postmark::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        let Some(db_err) = err.as_database_error() else {
            return Self::Database(err);
        };

        match (db_err.code().as_deref(), db_err.constraint()) {
            (Some("23505"), Some("email_verification_email_address_key")) => Error::EmailInUse,

            _ => Self::Database(err),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(match &self {
                Self::UserNotExists => StatusCode::NOT_FOUND,
                Self::EmailInUse => StatusCode::CONFLICT,
                Self::NoVerificationMatch => StatusCode::NOT_FOUND,
                Self::Database(error) => internal_error_old(error),
                Self::Postmark(error) => internal_error_old(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
struct CreateInfo {
    email_address: EmailAddress,
}

#[axum::debug_handler]
async fn create(
    user_session: UserSession,
    user_verifiaction: State<UserVerification>,
    form: Json<CreateInfo>,
) -> impl IntoResponse {
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

#[derive(Debug, Deserialize)]
struct DeleteInfo {
    email_address: EmailAddress,
    proof_token: VerificationToken,
}

#[axum::debug_handler]
async fn delete(
    user_session: UserSession,
    app_state: State<App>,
    delete_info: Json<DeleteInfo>,
) -> Result<()> {
    let user_id = user_session.get_user_id().await;
    let email_address = &delete_info.email_address;
    let proof_token = &delete_info.proof_token;

    let rows_affected = query!(
        "
        DELETE FROM _user.email_verification
            WHERE
                id = $1
                    AND
                email_address = $2
                    AND
                proof_token = $3
        ;
        ",
        user_id,
        email_address.as_str(),
        proof_token.to_string()
    )
    .execute(app_state.db())
    .await?
    .rows_affected();

    match rows_affected {
        1 => {
            query!(
                "
                UPDATE _user.account
                    SET
                        email_address = $2,
                        email_verified_on = $3
                    WHERE
                        id = $1
                ;
                ",
                user_id,
                email_address.as_str(),
                chrono::Utc::now()
            )
            .execute(app_state.db())
            .await?;

            Ok(())
        }

        0 => Err(Error::NoVerificationMatch),

        rows_affected => {
            error!("Dropped {rows_affected} during email validation!");

            Ok(())
        }
    }
}
