use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Solution {
    pub id: u64,
    pub created_at: DateTime<Utc>
}
