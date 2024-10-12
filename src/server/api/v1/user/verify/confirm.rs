use crate::{
    util::{EmailAddress, VerificationToken},
    server::{internal_error, state::AppState},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tower_sessions::Session;

#[derive(Debug, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    #[error("no verification match")]
    NoVerificationMatch,

    #[error("internal server error")]
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
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(match &self {
                Error::NoVerificationMatch => StatusCode::NOT_FOUND,
                Error::Database(error) => internal_error(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Info {
    email_address: EmailAddress,
    proof_token: VerificationToken,
}

#[axum::debug_handler]
pub async fn handler(
    session: Session,
    state: State<AppState>,
    body: Json<Info>,
) -> Result<(), Error> {
    let email_address = &body.email_address;
    let proof_token = &body.proof_token;
    let user_id = crate::server::state::get_user_id(&session).await;

    let rows_affected = query!(
        "
        DELETE FROM auth.email_verification
            WHERE
                user_id = $1
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
                chrono::Utc::now()
            )
            .execute(state.db())
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
