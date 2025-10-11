pub mod create;
pub mod delete;
pub mod read;
pub mod read_all;
pub mod update;

use crate::state::AppState;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct AccountLedger {
    db: crate::Datastore,
}

impl axum::extract::FromRef<AppState> for AccountLedger {
    fn from_ref(app: &AppState) -> Self {
        Self { db: app.db.clone() }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ACCOUNT_KIND")]
pub enum AccountKind {
    /// Asset; Cash; Checking
    AC_CHECKING,
    /// Asset; Cash; Saving
    AC_SAVING,
    /// Asset; Cash; Prepaid
    AC_PREPAID,
    /// Asset; Cash; Mobile Payments
    AC_MOBILE_PAYMENTS,
    /// Asset; Cash; Cash Management
    AC_CASH_MANAGEMENT,
    /// Asset; Investment; Brokerage
    AI_BROKERAGE,
    /// Asset; Investment; Roth IRA
    AI_ROTH_IRA,
    /// Asset; Investment; Traditional IRA
    AI_TRADITIONAL_IRA,
    /// Asset; Investment; SEP IRA
    AI_SEP_IRA,
    /// Asset; Investment; Traditional 401k
    AI_TRADITIONAL_401k,
    /// Asset; Investment; Roth 401k
    AI_ROTH_401k,
    /// Asset; Investment; 529 Plan
    AI_529_PLAN,
    /// Asset; Other
    A_OTHER,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountRecord {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub name: String,
    pub kind: AccountKind,
    pub description: Option<String>,
}
