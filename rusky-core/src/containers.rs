use crate::commands::CommandManager;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CommandManagerContainer;
impl TypeMapKey for CommandManagerContainer {
    type Value = Arc<Mutex<CommandManager>>;
}
