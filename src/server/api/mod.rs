use crate::server::state::AppState;
use chrono::{DateTime, Utc};
use uuid::Uuid;

mod v1;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().nest("/v1", v1::router())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountKind {
    Equity,
    Asset,
    Liability,
    Income,
    Expense,
}

impl AccountKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountKind::Equity => "EQUITY",
            AccountKind::Asset => "ASSET",
            AccountKind::Liability => "LIABILITY",
            AccountKind::Income => "INCOME",
            AccountKind::Expense => "EXPENSE",
        }
    }
}

impl std::fmt::Display for AccountKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<String> for AccountKind {
    fn from(value: String) -> Self {
        match value.as_str() {
            "EQUITY" => Self::Equity,
            "ASSET" => Self::Asset,
            "LIABILITY" => Self::Liability,
            "INCOME" => Self::Income,
            "EXPENSE" => Self::Expense,

            value => panic!("not a variant: {value}"),
        }
    }
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
