use askama_web::WebTemplate;
use axum::{Router, extract::State, response::IntoResponse, routing};
use uuid::Uuid;

use crate::{
    server::{UserSession, internal_error},
    state::{AppState, ledger::payee::PayeeLedger},
};

#[derive(askama::Template)]
#[template(path = "pages/payees.html")]
struct PayeesTemplate {}

struct PayeeListItem {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(askama::Template)]
#[template(path = "partials/payees/list.html")]
struct PayeeListTemplate {
    payee_list_items: Box<[PayeeListItem]>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            routing::get(|| async { WebTemplate(PayeesTemplate {}) }),
        )
        .route("/list", routing::get(_list_all))
}

async fn _list_all(
    user_session: UserSession,
    State(payee_ledger): State<PayeeLedger>,
) -> impl IntoResponse {
    match payee_ledger.read_all(user_session.id()).await {
        Ok(payees) => {
            let payee_list_items = payees
                .into_iter()
                .map(|record| PayeeListItem {
                    id: record.id,
                    name: record.name,
                    description: record.description,
                })
                .collect::<Box<[PayeeListItem]>>();

            WebTemplate(PayeeListTemplate { payee_list_items }).into_response()
        }

        Err(error) => internal_error(error).into_response(),
    }
}
