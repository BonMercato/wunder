use crate::prelude::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub api_key: String,
    pub pull_order_settings: PullOrderSettings,
}

#[derive(Debug, Deserialize)]
pub struct PullOrderSettings {
    pub order_state_codes: Vec<String>,
    pub order_path: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        config::Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()?
            .try_deserialize()
            .map_err(Into::into)
    }
}

impl PullOrderSettings {
    pub fn order_state_codes(&self) -> String {
        self.order_state_codes.join(",")
    }
}