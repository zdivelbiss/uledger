mod account;
mod commodity;
mod payee;
mod transaction;

use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct Ledger(Arc<Pool<Postgres>>);

impl From<Arc<Pool<Postgres>>> for Ledger {
    fn from(value: Arc<Pool<Postgres>>) -> Self {
        Self(value)
    }
}

impl Ledger {
    pub async fn create_account(
        &self,
        name: &str,
        kind: AccountKind,
        description: Option<&str>,
    ) -> Result<Account> {
        todo!()
    }

    pub async fn get_accounts(&self) -> Result<Box<[Account]>> {
        todo!()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Account {
    id: Uuid,
    created: DateTime<FixedOffset>,
    updated: DateTime<FixedOffset>,
    kind: AccountKind,
    name: String,
    description: Option<String>,
}

impl Account {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn created(&self) -> DateTime<FixedOffset> {
        self.created
    }

    pub fn updated(&self) -> DateTime<FixedOffset> {
        self.updated
    }

    pub fn kind(&self) -> AccountKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum AccountKind {
    #[serde(rename = "EQUITY")]
    Equity,

    #[serde(rename = "ASSET")]
    Asset,

    #[serde(rename = "LIABILITY")]
    Liability,

    #[serde(rename = "INCOME")]
    Income,

    #[serde(rename = "EXPENSE")]
    Expense,
}
