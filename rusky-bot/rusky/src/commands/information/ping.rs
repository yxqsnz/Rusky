use crate::command;
use rusky_core::{commands::CommandContext, Result};
#[command]
#[description("view my owo ping!")]
async fn ping(mut context: CommandContext) -> Result<()> {
    context.reply("pong!").await?;
    Ok(())
}
