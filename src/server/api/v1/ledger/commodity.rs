//! TODO use `rows_affected` to ensure IDs are actually affected

use crate::server::{
    api::{Commodity, CommodityInfo},
    internal_error_old,
    state::App,
    UserSession,
};
use axum::{
    extract::{Form, Json, Path, State},
    http::StatusCode,
    routing, Router,
};
use uuid::Uuid;

pub fn router() -> Router<App> {
    Router::new()
        .route("/", routing::get(get_all))
        .route("/", routing::post(create))
        .route("/:id", routing::get(read))
        .route("/:id", routing::put(update))
        .route("/:id", routing::delete(delete))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("account already exists")]
    Duplicate,

    #[error("internal server error")]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        let Some(db_error) = error.as_database_error() else {
            return Self::Database(error);
        };

        match (db_error.code().as_deref(), db_error.constraint()) {
            (Some("23505"), Some("accounts_user_id_kind_name_key")) => Error::Duplicate,

            _ => Self::Database(error),
        }
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(match &self {
                Self::Duplicate => StatusCode::CONFLICT,
                Self::Database(error) => internal_error_old(error),
            })
            .body(self.to_string().into())
            .unwrap()
    }
}

type Result<T> = std::result::Result<T, Error>;

async fn get_all(user_session: UserSession, app_state: State<App>) -> Result<Json<Vec<Commodity>>> {
    let user_id = user_session.get_user_id().await;

    let commodities = query_as!(
        Commodity,
        "
        SELECT id, created, name, format
            FROM _ledger..commodity
            WHERE
                user_id = $1
        ;
        ",
        user_id
    )
    .fetch_all(app_state.db())
    .await?;

    Ok(Json::from(commodities))
}

async fn create(
    user_session: UserSession,
    app_state: State<App>,
    commodity_info: Form<CommodityInfo>,
) -> Result<()> {
    let user_id = user_session.get_user_id().await;
    let commodity_name = commodity_info.name.as_str();
    let commodity_format = commodity_info.format.as_str();

    query!(
        "
        INSERT INTO _ledger..commodity
                (user_id, name, format)
            VALUES
                ($1, $2, $3)
        ;
        ",
        user_id,
        commodity_name,
        commodity_format
    )
    .execute(app_state.db())
    .await?;

    Ok(())
}

async fn read(
    user_session: UserSession,
    app_state: State<App>,
    commodity_id: Path<Uuid>,
) -> Result<Json<Commodity>> {
    let user_id = user_session.get_user_id().await;
    let commodity_id = *commodity_id;

    let commodity = query_as!(
        Commodity,
        "
        SELECT id, created, name, format
            FROM _ledger..commodity
            WHERE
                user_id = $2
                    AND
                id = $1
        ;
        ",
        user_id,
        commodity_id
    )
    .fetch_one(app_state.db())
    .await?;

    Ok(Json::from(commodity))
}

async fn update(
    user_session: UserSession,
    app_state: State<App>,
    commodity_id: Path<Uuid>,
    commodity_info: Json<CommodityInfo>,
) -> Result<()> {
    let user_id = user_session.get_user_id().await;
    let commodity_id = *commodity_id;
    let commodity_name = commodity_info.name.as_str();
    let commodity_format = commodity_info.format.as_str();

    query!(
        "
        UPDATE _ledger..commodity
            SET
                name = $3,
                format = $4
            WHERE
                user_id = $2
                    AND
                id = $1
        ;
        ",
        user_id,
        commodity_id,
        commodity_name,
        commodity_format
    )
    .execute(app_state.db())
    .await?;

    Ok(())
}

async fn delete(
    user_session: UserSession,
    app_state: State<App>,
    commodity_id: Path<Uuid>,
) -> Result<()> {
    let user_id = user_session.get_user_id().await;
    let commodity_id = *commodity_id;

    query!(
        "
        DELETE FROM _ledger..commodity
            WHERE
                user_id = $2
                    AND
                id = $1
        ;
        ",
        user_id,
        commodity_id
    )
    .execute(app_state.db())
    .await?;

    Ok(())
}
