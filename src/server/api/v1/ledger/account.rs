//! TODO use `rows_affected` to ensure IDs are actually affected

use crate::server::{htmx::HtmxInfo, internal_error_old, state::App, UserSession};
use axum::{
    extract::{Form, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Router,
};
use lib::ledger::account::{Info, Kind, Record};
use uuid::Uuid;

pub fn router() -> Router<App> {
    Router::new()
        .route("/", routing::get(get_all))
        .route("/", routing::post(create))
        .route("/:id", routing::get(read))
        .route("/:id", routing::put(update))
        .route("/:id", routing::delete(delete))
}

#[derive(askama::Template)]
#[template(path = "partials/account/list-all.html")]
pub struct AccountListTemplate {
    accounts: Box<[AccountInfo]>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("account already exists")]
    Duplicate,

    #[error("internal server error")]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        let Some(db_err) = err.as_database_error() else {
            return Self::Database(err);
        };

        match (db_err.code().as_deref(), db_err.constraint()) {
            (Some("23505"), Some("accounts_user_id_kind_name_key")) => Error::Duplicate,

            _ => Self::Database(err),
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

async fn get_all(
    user_session: UserSession,
    htmx: Option<HtmxInfo>,
    app_state: State<App>,
) -> Result<impl IntoResponse> {
    let user_id = user_session.get_user_id().await;

    if htmx.is_some() {
        let accounts = query_as!(
            AccountInfo,
            "
            SELECT kind AS \"kind: AccountKind\", name, description
                FROM _ledger..account
                WHERE
                    user_id = $1
            ;
            ",
            user_id
        )
        .fetch_all(app_state.db())
        .await?
        .into_boxed_slice();

        Ok(AccountListTemplate { accounts }.into_response())
    } else {
        let accounts = query_as!(
            Account,
            "
            SELECT id, created, kind AS \"kind: AccountKind\", name, description
                FROM _ledger..account
                WHERE
                    user_id = $1
            ;
            ",
            user_id
        )
        .fetch_all(app_state.db())
        .await?;

        Ok(Json::from(accounts).into_response())
    }
}

async fn create(
    user_session: UserSession,
    app_state: State<App>,
    account_info: Form<AccountInfo>,
) -> Result<()> {
    let user_id = user_session.get_user_id().await;
    let account_kind = account_info.kind;
    let account_name = account_info.name.as_str();
    let account_description = account_info.description.as_deref();

    query!(
        "
        INSERT INTO _ledger..account
                (user_id, kind, name, description)
            VALUES
                ($1, $2, $3, $4)
        ;
        ",
        user_id,
        account_kind as _,
        account_name,
        account_description
    )
    .execute(app_state.db())
    .await?;

    Ok(())
}

async fn read(
    user_session: UserSession,
    app_state: State<App>,
    account_id: Path<Uuid>,
) -> Result<Json<Account>> {
    let user_id = user_session.get_user_id().await;
    let account_id = *account_id;

    let account = query_as!(
        Account,
        "
        SELECT id, created, kind AS \"kind: AccountKind\", name, description
            FROM _ledger..account
            WHERE
                user_id = $2
                    AND
                id = $1
        ;
        ",
        user_id,
        account_id,
    )
    .fetch_one(app_state.db())
    .await?;

    Ok(Json::from(account))
}

async fn update(
    user_session: UserSession,
    app_state: State<App>,
    account_id: Path<Uuid>,
    account_info: Json<AccountInfo>,
) -> Result<()> {
    let user_id = user_session.get_user_id().await;
    let account_id = *account_id;
    let account_kind = account_info.kind;
    let account_name = account_info.name.as_str();
    let account_description = account_info.description.as_deref();

    query!(
        "
        UPDATE _ledger..account
            SET
                kind = $3,
                name = $4,
                description = $5
            WHERE
                user_id = $2
                    AND
                id = $1
        ;
        ",
        user_id,
        account_id,
        account_kind as _,
        account_name,
        account_description
    )
    .execute(app_state.db())
    .await?;

    Ok(())
}

async fn delete(
    user_session: UserSession,
    app_state: State<App>,
    account_id: Path<Uuid>,
) -> Result<()> {
    let user_id = user_session.get_user_id().await;
    let account_id = *account_id;

    query!(
        "
        DELETE FROM _ledger..account
            WHERE
                user_id = $2
                    AND
                id = $1
        ;
        ",
        user_id,
        account_id
    )
    .execute(app_state.db())
    .await?;

    Ok(())
}
