use crate::{
    server::{UserSession, internal_error},
    state::{AppState, ledger::account::AccountLedger},
};
use askama_web::WebTemplate;
use axum::{Router, extract::State, response::IntoResponse, routing};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(askama::Template)]
#[template(path = "pages/accounts.html")]
struct AccountsTemplate {}

struct AccountListItem {
    pub id: Uuid,
    pub created: NaiveDate,
    pub name: String,
    pub kind: &'static str,
    pub description: Option<String>,
}

#[derive(askama::Template)]
#[template(path = "partials/accounts/list.html")]
struct AccountsListTemplate {
    account_list_items: Box<[AccountListItem]>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            routing::get(|| async { WebTemplate(AccountsTemplate {}) }),
        )
        .route("/list", routing::get(_list_all))
}

async fn _list_all(
    user_session: UserSession,
    State(account_ledger): State<AccountLedger>,
) -> impl IntoResponse {
    match account_ledger.read_all(user_session.id()).await {
        Ok(accounts) => {
            let account_list_items = accounts
                .into_iter()
                .map(|record| AccountListItem {
                    id: record.id,
                    created: record.created.date_naive(),
                    name: record.name,
                    kind: record.kind.friendly_name(),
                    description: record.description,
                })
                .collect::<Box<[AccountListItem]>>();

            WebTemplate(AccountsListTemplate { account_list_items }).into_response()
        }

        Err(error) => internal_error(error).into_response(),
    }
}
