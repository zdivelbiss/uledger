use crate::{
    server::{UserSession, internal_error},
    state::{
        AppState,
        ledger::{
            account::{AccountLedger, AccountRecord},
            payee::{PayeeLedger, PayeeRecord},
        },
    },
    util::Currency,
};
use askama_web::WebTemplate;
use axum::{
    Router,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect},
    routing::get,
};

#[derive(askama::Template)]
#[template(path = "singles/register.html")]
pub struct RegisterTemplate {}

#[derive(askama::Template)]
#[template(path = "singles/login.html")]
pub struct LoginTemplate {}

#[derive(askama::Template)]
#[template(path = "pages/accounts.html")]
struct AccountsTemplate {}

#[derive(askama::Template)]
#[template(path = "pages/payees.html")]
struct PayeesTemplate {}

#[derive(askama::Template)]
#[template(path = "pages/transactions.html")]
struct TransactionsTemplate {
    accounts: Box<[AccountRecord]>,
    payees: Box<[PayeeRecord]>,
    currencies: &'static [Currency],
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Authenticated
        .route("/", get(|| async { Redirect::temporary("/accounts") }))
        .route(
            "/accounts",
            get(|| async { WebTemplate(AccountsTemplate {}) }),
        )
        .route("/payees", get(|| async { WebTemplate(PayeesTemplate {}) }))
        .route(
            "/transactions",
            get(
                |user_session: UserSession,
                 State(account_ledger): State<AccountLedger>,
                 State(payee_ledger): State<PayeeLedger>| async move {
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
                },
            ),
        )
        .layer(axum::middleware::from_fn(check_user_authenticated))
        // Unauthenticated
        .route(
            "/register",
            get(|| async { WebTemplate(RegisterTemplate {}) }),
        )
        .route("/login", get(|| async { WebTemplate(LoginTemplate {}) }))
}

async fn check_user_authenticated(
    user_session: Option<UserSession>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    if user_session.is_some() {
        next.run(request).await
    } else {
        Redirect::temporary("/login").into_response()
    }
}
