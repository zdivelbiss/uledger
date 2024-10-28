use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Commodity {
    id: Uuid,
    created: DateTime<Utc>,
    name: String,
    format: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CommodityInfo {
    name: String,
    format: String,
}
