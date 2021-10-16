use crate::Result;
use dotenv::dotenv;
use figment::{
    providers::{Env, Format, Json, Toml, Yaml},
    Figment,
};
use serde::Deserialize;
use tracing::debug;
#[derive(Deserialize)]
pub struct DiscordConfig {
    pub id: u64,
    pub token: String,
    pub auto_sharded: bool,
    pub shard_amount: Option<usize>,
}
#[derive(Deserialize)]
pub struct RuskyConfig {
    pub discord: DiscordConfig,
}
impl RuskyConfig {
    pub fn collect_from_world() -> Result<Self> {
        debug!("collecting config from real world... ");
        dotenv().ok();
        Ok(Figment::new()
            .merge(Toml::file("Rusky.toml"))
            .merge(Env::prefixed("RUSKY_"))
            .merge(Json::file("Rusky.json")) 
            .merge(Yaml::file("Rusky.yml"))
            .extract()?)
    }
}
