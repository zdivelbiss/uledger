use crate::server::state::AppState;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

mod v1;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().nest("/v1", v1::router())
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    IntoPrimitive,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
)]
#[repr(i16)]
pub enum AccountKind {
    Equity = 0,
    Asset = 1,
    Liability = 2,
    Income = 3,
    Expense = 4,

    #[default]
    Unknown = -1,
}

impl std::fmt::Display for AccountKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
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
