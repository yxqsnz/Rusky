#[derive(Default)]
pub struct BotBuilder {
    pub token: Option<String>,
    pub application_id: Option<u64>,
}
pub struct BotConfigBuilded {
    pub token: String,
    pub application_id: u64,
}
impl BotBuilder {
    pub fn token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn id(mut self, id: u64) -> Self {
        self.application_id = Some(id);
        self
    }

    pub fn build(self) -> BotConfigBuilded {
        BotConfigBuilded {
            token: self.token.expect("expected a Token."),
            application_id: self.application_id.expect("expected a ID."),
        }
    }
}
