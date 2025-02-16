use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub repositories: Vec<Repository>,
}
