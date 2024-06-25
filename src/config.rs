use std::fs;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub firefly_url: String,
    pub firefly_pat: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;

        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}
