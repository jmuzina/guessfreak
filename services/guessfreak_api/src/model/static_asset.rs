use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AssetType {
    IMAGE,
    VIDEO,
    AUDIO
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StaticAsset {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub asset_type: AssetType,
    pub path: String,
    pub description: Option<String>
}
