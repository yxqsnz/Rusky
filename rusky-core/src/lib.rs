use self::commands::CommandManager;
use crate::builders::bot::BotBuilder;
use serenity::{prelude::EventHandler, Client};
use std::{error::Error as StdError, result::Result as StdResult, sync::Arc};
use tokio::sync::Mutex;
pub type Error = Box<dyn StdError + Sync + Send>;
pub type Result<T> = StdResult<T, Error>;

//* mods
pub mod builders;
pub mod commands;
pub mod consts;
pub mod containers;
pub mod macros;
//*
/// the Rusky bot Core
pub struct Bot {
    pub client: Client,
    pub commands: CommandManager,
}
impl Bot {
    /// create a new bot.
    pub async fn new<F, H>(mut f: F, h: H) -> Result<Self>
    where
        F: FnMut(BotBuilder) -> BotBuilder,
        H: EventHandler,
        H: 'static, {
        let cfg = f(BotBuilder::default()).build();
        Ok(Self {
            commands: CommandManager::default(),
            client: Client::builder(cfg.token)
                .application_id(cfg.application_id)
                .event_handler(h)
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
        {
            let mut data = self.client.data.write().await;
            data.insert::<containers::CommandManagerContainer>(Arc::new(Mutex::new(self.commands)));
        }
        self.client.start_autosharded().await?;
        Ok(())
    }
}
