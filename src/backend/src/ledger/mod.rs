mod account;
mod commodity;
mod payee;
mod transaction;

use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use postgrest::Postgrest;
use uuid::Uuid;

pub struct Ledger {
    client: Postgrest,
}

impl Ledger {
    const ACCOUNTS: &str = "accounts";

    pub fn open(endpoint: &str, apikey: &str, servicekey: Option<&str>) -> Self {
        let mut client = postgrest::Postgrest::new(endpoint).insert_header("apikey", apikey);

        if let Some(servicekey) = servicekey {
            client = client.insert_header("Authorization", format!("Bearer {servicekey}"));
        }

        Self { client }
    }

    pub async fn create_account(
        &self,
        name: &str,
        kind: AccountKind,
        description: Option<&str>,
    ) -> Result<Account> {
        #[derive(Debug, serde::Serialize)]
        struct CreateAccount<'a> {
            kind: AccountKind,
            name: &'a str,
            description: Option<&'a str>,
        }

        let create_account = CreateAccount {
            kind,
            name,
            description,
        };
        let create_account_json = serde_json::to_string(&create_account)?;

        let response = self
            .client
            .from(Self::ACCOUNTS)
            .insert(create_account_json)
            .execute()
            .await?
            .error_for_status()?;

        let response_text = response.text().await?;
        trace!("Create Account Response: {response_text:?}");

        // The response is technically an array, but it's only ever one item.
        let account = serde_json::from_str::<Account>(response_text.trim_matches(['[', ']']))?;

        Ok(account)
    }

    pub async fn get_accounts(&self) -> Result<Box<[Account]>> {
        let response = self
            .client
            .from(Self::ACCOUNTS)
            .select("*")
            .execute()
            .await?;
        let response_text = response.text().await?;

        let accounts = serde_json::from_str(&response_text)?;

        Ok(accounts)
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
