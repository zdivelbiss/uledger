use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::Display,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "ACCOUNT_KIND", rename_all = "UPPERCASE")]
pub enum AccountKind {
    Equity,
    Asset,
    Liability,
    Income,
    Expense,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AccountRecord {
    id: Uuid,
    created: DateTime<Utc>,
    kind: AccountKind,
    name: String,
    description: Option<String>,
}
