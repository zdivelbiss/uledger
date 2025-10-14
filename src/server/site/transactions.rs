use askama_web::WebTemplate;
use axum::{Router, extract::State, response::IntoResponse, routing};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    server::{UserSession, internal_error},
    state::{
        AppState,
        ledger::{
            account::{AccountLedger, AccountRecord},
            payee::{PayeeLedger, PayeeRecord},
        },
    },
    util::{Currency, CurrencyAmount, CurrencyCode},
};

#[derive(askama::Template)]
#[template(path = "pages/transactions.html")]
struct TransactionsTemplate {
    accounts: Box<[AccountRecord]>,
    payees: Box<[PayeeRecord]>,
    currencies: &'static [Currency],
}

struct TransactionListItem {
    pub id: Uuid,
    pub occurred_on: NaiveDate,
    pub account: Uuid,
    pub account_name: String,
    pub payee: Uuid,
    pub payee_name: String,
    pub amount: String,
    pub description: Option<String>,
}

#[derive(askama::Template)]
#[template(path = "partials/transactions/list.html")]
struct TransactionsListTemplate {
    transaction_list_items: Box<[TransactionListItem]>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(_page))
        .route("/list", routing::get(_list_all))
}

async fn _page(
    user_session: UserSession,
    State(account_ledger): State<AccountLedger>,
    State(payee_ledger): State<PayeeLedger>,
) -> impl IntoResponse {
    let accounts = match account_ledger.read_all(user_session.id()).await {
        Ok(accounts) => accounts,
        Err(error) => return internal_error(error).into_response(),
    };
    let payees = match payee_ledger.read_all(user_session.id()).await {
        Ok(payees) => payees,
        Err(error) => return internal_error(error).into_response(),
    };

    WebTemplate(TransactionsTemplate {
        accounts,
        payees,
        currencies: Currency::get_all(),
    })
    .into_response()
}

async fn _list_all(
    user_session: UserSession,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    #[derive(Debug, Serialize, Deserialize, sqlx::Type)]
    struct TransactionListRecord {
        pub id: Uuid,
        pub occurred_on: NaiveDate,
        pub account: Uuid,
        pub account_name: String,
        pub payee: Uuid,
        pub payee_name: String,
        pub currency: CurrencyCode,
        pub amount: CurrencyAmount,
        pub description: Option<String>,
    }

    let result = query_as!(
        TransactionListRecord,
        "
        SELECT  t.id AS \"id: _\",
                t.occurred_on AS \"occurred_on: _\",
                a.id AS \"account: _\",
                a.name AS \"account_name: _\",
                p.id AS \"payee: _\",
                p.name AS \"payee_name: _\",
                t.currency AS \"currency: CurrencyCode\",
                t.amount AS \"amount: CurrencyAmount\",
                t.description AS \"description: _\"
            FROM _ledger.transaction t
            LEFT JOIN _ledger.account a
                ON t.account = a.id
            LEFT JOIN _ledger.payee p
                ON t.payee = p.id
            WHERE t.user_id = $1
        ;
        ",
        user_session.id()
    )
    .fetch_all(app_state.db_pool())
    .await
    .map(Vec::into_boxed_slice);

    match result {
        Ok(records) => {
            let transaction_list_items = records
                .into_iter()
                .map(|record| {
                    let currency = Currency::get(record.currency);
                    let amount = currency.parse(record.amount);

                    TransactionListItem {
                        id: record.id,
                        occurred_on: record.occurred_on,
                        account: record.account,
                        account_name: record.account_name,
                        payee: record.payee,
                        payee_name: record.payee_name,
                        description: record.description,
                        amount,
                    }
                })
                .collect::<Box<[TransactionListItem]>>();

            WebTemplate(TransactionsListTemplate {
                transaction_list_items,
            })
            .into_response()
        }

        Err(error) => internal_error(error).into_response(),
    }
}
