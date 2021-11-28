use rusky::{commands::*, *};
use rusky_core::{containers::CommandManagerContainer, Bot, Result};
use rusky_utils::config::RuskyConfig;
use serenity::{
    async_trait,
    model::{interactions::Interaction, prelude::Ready},
    prelude::*,
};

pub struct ReventHandler;

#[async_trait]
impl EventHandler for ReventHandler {
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        {
            let data = &context.data.read().await;
            {
                let arc = data
                    .get::<CommandManagerContainer>()
                    .expect("command manager.");
                {
                    let mut manager = arc.lock().await;
                    {
                        manager.execute(context.clone(), interaction).await.ok();
                    }
                }
            }
        }
    }

    async fn ready(&self, context: Context, _: Ready) {
        {
            let data = &context.data.read().await;
            {
                let arc = data
                    .get::<CommandManagerContainer>()
                    .expect("command manager.");
                {
                    let mut manager = arc.lock().await;
                    {
                        manager.update_commands(&context.clone()).await.ok();
                    }
                }
            }
        }
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    println!("Rusky Bot - 🌅 A new bot for discord 🌅");
    logging::setup()?;
    let mut bot = Bot::new(
        |b| {
            let cfg = RuskyConfig::collect_from_world().expect("config");
            b.id(cfg.discord.id).token(cfg.discord.token)
        },
        ReventHandler,
    )
    .await?;
    bot.commands.add(ping::PING_COMMAND);
    bot.start().await?;

    Ok(())
}
