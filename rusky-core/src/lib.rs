use crate::builders::bot::BotConfigBuilded;
use serenity::Client;
use std::{error::Error as StdError, result::Result as StdResult};
pub type Error = Box<dyn StdError + Sync + Send>;
pub type Result<T> = StdResult<T, Error>;

//* mods
pub mod builders;
pub mod commands;
pub mod macros;
//*
/// the Rusky bot Core
pub struct Bot {
    pub client: Client,
}
impl Bot {
    /// create a new bot.
    pub async fn new(cfg: BotConfigBuilded) -> Result<Self> {
        Ok(Self {
            client: Client::builder(cfg.token)
                .application_id(cfg.application_id)
                .event_handler(cfg.event_handler)
                .await?,
        })
    }

    /// start bot with N Shards.
    pub async fn start_shards(mut self, total_shards: u64) -> Result<()> {
        self.client.start_shards(total_shards).await?;
        Ok(())
    }

    /// start bot with automatic shard amount.
    pub async fn start(mut self) -> Result<()> {
        self.client.start_autosharded().await?;
        Ok(())
    }
}
