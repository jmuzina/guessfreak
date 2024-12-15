use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::model::static_asset::StaticAsset;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SolutionGuess {
    pub solution_id: u64,
    pub guess: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SolutionType {
    pub id: u64,
    pub name: String,
    pub label: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Solution {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub solution_type:SolutionType
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SolutionChance {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub solution: Solution,
    pub text_html: String,
    pub static_asset: Option<StaticAsset>
}