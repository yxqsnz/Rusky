use crate::{consts, Result};
use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateApplicationCommandOption},
    client::Context,
    model::interactions::{
        application_command::ApplicationCommand,
        Interaction,
        InteractionResponseType,
    },
};
use std::{collections::HashMap, fmt::Display};
#[async_trait]
pub trait SlashCommand {
    fn data(&self) -> SlashCommandData;
    async fn execute(&self, context: CommandContext) -> Result<()>;
}
#[derive(Clone)]
pub struct SlashCommandData {
    pub name: String,
    pub description: Option<String>,
    pub options: Option<Vec<CreateApplicationCommandOption>>,
}
/// the command context
#[derive(Clone)]
pub struct CommandContext {
    pub bot: Context,
    pub interaction: Interaction,
}
impl CommandContext {
    pub async fn reply(&mut self, content: impl Display) -> Result<()> {
        if let Interaction::ApplicationCommand(app) = &self.interaction {
            app.create_interaction_response(&self.bot, |b| {
                b.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|m| {
                        m.create_embed(|e| e.description(content).color(consts::BLUE))
                    })
            })
            .await?;
        }
        Ok(())
    }
}
#[derive(Default)]
pub struct CommandManager {
    pub commands: HashMap<String, Box<dyn SlashCommand + Sync + Send>>,
}
impl CommandManager {
    pub fn add<Command>(&mut self, command: Command) -> &mut Self
    where
        Command: SlashCommand + Sync + Send,
        Command: 'static, {
        self.commands.insert(command.data().name, Box::new(command));
        self
    }

    pub async fn update_commands(&mut self, c: &Context) -> Result<()> {
        tracing::info!("update commands > getting global commands...");
        let commands = ApplicationCommand::get_global_application_commands(c).await?;
        let add_commands;
        if commands.len() != self.commands.len() {
            tracing::info!("update commands > can update.");
            add_commands = commands.len() < self.commands.len();
            ApplicationCommand::set_global_application_commands(c, |set_commands| {
                for (name, command) in &self.commands {
                    if add_commands && !commands.iter().any(|x| x.name.eq(name)) {
                        set_commands.add_application_command(
                            CreateApplicationCommand::default()
                                .name(name)
                                .description(
                                    command
                                        .data()
                                        .description
                                        .unwrap_or_else(|| String::from("description not impl.")),
                                )
                                .set_options(command.data().options.unwrap_or_default())
                                .to_owned(),
                        );
                    } else {
                        for (name, command) in &self.commands {
                            set_commands.add_application_command(
                                CreateApplicationCommand::default()
                                    .name(name)
                                    .description(
                                        command.data().description.unwrap_or_else(|| {
                                            String::from("description not impl.")
                                        }),
                                    )
                                    .set_options(command.data().options.unwrap_or_default())
                                    .to_owned(),
                            );
                        }
                    }
                }
                set_commands
            })
            .await?;
        }
        tracing::info!("update command > done.");
        Ok(())
    }

    pub async fn execute(&mut self, context: Context, interaction: Interaction) -> Result<()> {
        if let Interaction::ApplicationCommand(app) = interaction.clone() {
            if let Some(command) = self.commands.get(&app.data.name) {
                if let Err(err) = command
                    .execute(CommandContext {
                        bot: context,
                        interaction,
                    })
                    .await
                {
                    tracing::error!("{}", err);
                }
            }
        }
        Ok(())
    }
}
