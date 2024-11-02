use chrono::{DateTime, NaiveDateTime, Utc};
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountRecord {
    pub id: Uuid,
    pub created: NaiveDateTime,
    pub kind: AccountKind,
    pub name: String,
    pub description: Option<String>,
}
