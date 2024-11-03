use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommodityRecord {
    id: Uuid,
    created: DateTime<Utc>,
    name: String,
    format: String,
}
