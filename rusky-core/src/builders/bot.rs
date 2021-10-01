use serenity::prelude::EventHandler;
pub struct BotBuilder {
    pub token: Option<String>,
    pub application_id: Option<u64>,
    pub event_handler: Option<Box<dyn EventHandler>>,
}
pub struct BotConfigBuilded {
    pub token: String,
    pub application_id: u64,
    pub event_handler: Box<dyn EventHandler>,
}
impl Default for BotBuilder {
    fn default() -> Self {
        Self {
            token: None,
            application_id: None,
            event_handler: None,
        }
    }
}
impl BotBuilder {
    pub fn token(&mut self, token: String) -> &mut Self {
        self.token = Some(token);
        self
    }

    pub fn id(&mut self, id: u64) -> &mut Self {
        self.application_id = Some(id);
        self
    }

    pub fn handler(&mut self, handler: Box<dyn EventHandler>) -> &mut Self {
        self.event_handler = Some(handler);
        self
    }

    pub fn build(self) -> BotConfigBuilded {
        BotConfigBuilded {
            token: self.token.expect("expected a Token."),
            application_id: self.application_id.expect("expected a ID."),
            event_handler: self.event_handler.expect("expected a EventHandler."),
        }
    }
}
