pub mod paging {
    use serde::{Deserialize, Deserializer, Serialize};

    pub static MAX_PAGE_LIMIT: usize = 100;
    pub static MIN_PAGE_LIMIT: usize = 1;

    // Clamps page request limit to a minimum of 1 and a maximum of 100
    fn clamp_limit<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where
        D: Deserializer<'de>
    {
        let limit = usize::deserialize(deserializer)?;
        Ok(limit.max(MIN_PAGE_LIMIT).min(MAX_PAGE_LIMIT))
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PagedRequest {
        pub start: usize,
        #[serde(default = "default_limit", deserialize_with = "clamp_limit")]
        pub limit: usize,
    }

    fn default_limit() -> usize {
        MAX_PAGE_LIMIT
    }
}