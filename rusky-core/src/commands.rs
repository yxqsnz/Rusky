use crate::Result;
use serenity::{
    async_trait,
    client::Context,
    model::interactions::application_command::{ApplicationCommand, ApplicationCommandOption},
};
#[async_trait]
pub trait SlashCommand {
    fn data(&self) -> SlashCommandData;
    async fn execute(&self, context: &CommandContext) -> Result<()>;
}
pub struct SlashCommandData {
    pub name: String,
    pub description: Option<String>,
    pub options: Option<Vec<ApplicationCommandOption>>,
}
/// the command context.
pub struct CommandContext {
    pub bot: Context,
    pub command: ApplicationCommand,
}
