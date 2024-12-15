use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PathRequest {
    pub path: String
}