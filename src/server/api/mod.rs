use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::server::state::AppState;

mod v1;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().nest("/v1", v1::router())
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, FromPrimitive, Serialize, Deserialize,
)]
#[serde(rename_all = "UPPERCASE")]
#[repr(i16)]
pub enum Kind {
    #[default]
    Equity = 0,
    Asset = 1,
    Liability = 2,
    Income = 3,
    Expense = 4,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Kind::Equity => "Equity",
            Kind::Asset => "Asset",
            Kind::Liability => "Liability",
            Kind::Income => "Income",
            Kind::Expense => "Expense",
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    kind: Kind,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    id: Uuid,
    created: DateTime<Utc>,
    kind: Kind,
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
