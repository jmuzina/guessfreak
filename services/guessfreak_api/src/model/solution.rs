use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::model::static_asset::StaticAsset;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Solution {
    pub id: u64,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SolutionChance {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub solution: Solution,
    pub text_html: String,
    pub static_asset: Option<StaticAsset>,
    pub chance_order: Option<u64>
}