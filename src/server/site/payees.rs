use askama_web::WebTemplate;
use axum::{Router, routing};

use crate::state::AppState;

#[derive(askama::Template)]
#[template(path = "pages/payees.html")]
struct PayeesTemplate {}

// #[derive(askama::Template)]
// #[template(path = "partials/accounts/list.html")]
// struct AccountsListTemplate {
//     accounts: Box<[AccountListItem]>,
// }

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/",
        routing::get(|| async { WebTemplate(PayeesTemplate {}) }),
    )
    // .route("/list", routing::get(_list_all))
}

// async fn _list_all(
//     user_session: UserSession,
//     State(account_ledger): State<AccountLedger>,
// ) -> impl IntoResponse {
//     match account_ledger.read_all(user_session.id()).await {
//         Ok(accounts) => {
//             let accounts = accounts
//                 .into_iter()
//                 .map(|record| AccountListItem {
//                     name: record.name,
//                     kind: record.kind.friendly_name(),
//                     description: record.description,
//                 })
//                 .collect::<Box<[AccountListItem]>>();

//             WebTemplate(AccountsListPartial { accounts }).into_response()
//         }

//         Err(error) => internal_error(error).into_response(),
//     }
// }
