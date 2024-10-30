use crate::server::{
    internal_error,
    state::{ledger::account::AccountLedger, App},
    UserSession,
};
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use lib::ledger::account::AccountKind;
use uuid::Uuid;

pub fn router() -> Router<App> {
    Router::new()
        .route(
            "/",
            get(
                |user_session: UserSession, ledger: State<AccountLedger>| async move {
                    let user_id = user_session.get_user_id().await;
                    match ledger.read_all(user_id).await {
                        Ok(_records) => todo!(),
                        Err(error) => internal_error(error).into_response(),
                    }
                },
            ),
        )
        .route(
            "/",
            post({
                #[derive(Debug, Deserialize)]
                struct Info {
                    kind: AccountKind,
                    name: String,
                    description: Option<String>,
                }

                |user_session: UserSession, ledger: State<AccountLedger>, info: Form<Info>| async move {
                    use crate::server::state::ledger::account::create::Error;
                    
                    let user_id = user_session.get_user_id()
                    .await;
                    match ledger.create(user_id, info.kind, info.name.as_str(), info.description.as_deref()).await {
                        Ok(_record) => todo!(),

                        Err(Error::Duplicate) => (StatusCode::CONFLICT, "account already exists").into_response(),
                    
                        Err(error) => internal_error(error).into_response()
                    }
                }
            }),
        )
    .route("/:id", get(
        |user_session: UserSession, ledger: State<AccountLedger>, id: Path<Uuid>| async
        move {
            use crate::server::state::ledger::account::read::Error;

            let user_id = user_session.get_user_id().await;
            match ledger.read(user_id, *id).await {
                Ok(_record) => todo!(),

                Err(Error::NotFound) => (StatusCode::NOT_FOUND, "account not found").into_response(),

                Err(error) => internal_error(error).into_response()
            }
        }
    ))
    // .route("/:id", put(update))
    // .route("/:id", delete(delete))
}

// #[derive(askama::Template)]
// #[template(path = "partials/account/list-all.html")]
// pub struct AccountListTemplate {
//     accounts: Box<[AccountInfo]>,
// }


// async fn get_all(
//     user_session: UserSession,
//     htmx: Option<HtmxInfo>,
//     app_state: State<App>,
// ) -> Result<impl IntoResponse> {
//     let user_id = user_session.get_user_id().await;

//     if htmx.is_some() {
//         let accounts = query_as!(
//             AccountInfo,
//             "
//             SELECT kind AS \"kind: AccountKind\", name, description
//                 FROM _ledger..account
//                 WHERE
//                     user_id = $1
//             ;
//             ",
//             user_id
//         )
//         .fetch_all(app_state.db())
//         .await?
//         .into_boxed_slice();

//         Ok(AccountListTemplate { accounts }.into_response())
//     } else {
//         let accounts = query_as!(
//             Account,
//             "
//             SELECT id, created, kind AS \"kind: AccountKind\", name, description
//                 FROM _ledger..account
//                 WHERE
//                     user_id = $1
//             ;
//             ",
//             user_id
//         )
//         .fetch_all(app_state.db())
//         .await?;

//         Ok(Json::from(accounts).into_response())
//     }
// }

// async fn read(
//     user_session: UserSession,
//     app_state: State<App>,
//     account_id: Path<Uuid>,
// ) -> Result<Json<Account>> {
//     let user_id = user_session.get_user_id().await;
//     let account_id = *account_id;

//     let account = query_as!(
//         Account,
//         "
//         SELECT id, created, kind AS \"kind: AccountKind\", name, description
//             FROM _ledger..account
//             WHERE
//                 user_id = $2
//                     AND
//                 id = $1
//         ;
//         ",
//         user_id,
//         account_id,
//     )
//     .fetch_one(app_state.db())
//     .await?;

//     Ok(Json::from(account))
// }

// async fn update(
//     user_session: UserSession,
//     app_state: State<App>,
//     account_id: Path<Uuid>,
//     account_info: Json<AccountInfo>,
// ) -> Result<()> {
//     let user_id = user_session.get_user_id().await;
//     let account_id = *account_id;
//     let account_kind = account_info.kind;
//     let account_name = account_info.name.as_str();
//     let account_description = account_info.description.as_deref();

//     query!(
//         "
//         UPDATE _ledger..account
//             SET
//                 kind = $3,
//                 name = $4,
//                 description = $5
//             WHERE
//                 user_id = $2
//                     AND
//                 id = $1
//         ;
//         ",
//         user_id,
//         account_id,
//         account_kind as _,
//         account_name,
//         account_description
//     )
//     .execute(app_state.db())
//     .await?;

//     Ok(())
// }

// async fn delete(
//     user_session: UserSession,
//     app_state: State<App>,
//     account_id: Path<Uuid>,
// ) -> Result<()> {
//     let user_id = user_session.get_user_id().await;
//     let account_id = *account_id;

//     query!(
//         "
//         DELETE FROM _ledger..account
//             WHERE
//                 user_id = $2
//                     AND
//                 id = $1
//         ;
//         ",
//         user_id,
//         account_id
//     )
//     .execute(app_state.db())
//     .await?;

//     Ok(())
// }
