use crate::server::{
    internal_error,
    state::{get_user_id, AppState},
};
use axum::{
    extract::{Path, State},
    Json,
};
use tower_sessions::Session;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
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

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(match &self {
                Self::Database(error) => internal_error(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

pub async fn update(
    session: Session,
    state: State<AppState>,
    id: Path<Uuid>,
    info: Json<super::CommodityInfo>,
) -> Result<(), Error> {
    query!(
        "
        UPDATE ledger.commodities
            SET
                name = $3,
                format = $4
            WHERE
                id = $1
                    AND
                user_id = $2
        ;
        ",
        *id,
        get_user_id(&session).await,
        info.name,
        info.format
    )
    .execute(state.db())
    .await?;

    Ok(())
}
