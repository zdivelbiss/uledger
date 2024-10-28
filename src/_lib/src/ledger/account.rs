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
pub enum Kind {
    Equity,
    Asset,
    Liability,
    Income,
    Expense,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Info {
    kind: Kind,
    name: String,
    description: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Record {
    id: Uuid,
    created: DateTime<Utc>,
    kind: Kind,
    name: String,
    description: Option<String>,
}

impl From<Record> for Info {
    fn from(value: Record) -> Self {
        Self {
            kind: value.kind,
            name: value.name,
            description: value.description,
        }
    }
}
