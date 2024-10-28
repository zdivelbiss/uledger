use crate::server::state::App;
use chrono::{DateTime, Utc};
use uuid::Uuid;

mod v1;

pub fn router() -> axum::Router<App> {
    axum::Router::new().nest("/v1", v1::router())
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "ACCOUNT_KIND", rename_all = "UPPERCASE")]
pub enum AccountKind {
    Equity,
    Asset,
    Liability,
    Income,
    Expense,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    kind: AccountKind,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    id: Uuid,
    created: DateTime<Utc>,
    kind: AccountKind,
    name: String,
    description: Option<String>,
}

impl From<Account> for AccountInfo {
    fn from(value: Account) -> Self {
        Self {
            kind: value.kind,
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Commodity {
    id: Uuid,
    created: DateTime<Utc>,
    name: String,
    format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommodityInfo {
    name: String,
    format: String,
}
