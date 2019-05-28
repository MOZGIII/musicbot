use std::collections::HashSet;

use serenity::{
    framework::standard::Configuration,
    http,
    model::id::UserId,
};

pub struct StandardFrameworkConfig {
    owner_id: UserId,
    bot_id: UserId,
}

impl StandardFrameworkConfig {
    pub fn new(http: &http::Http) -> Result<Self, Box<std::error::Error>> {
        let info = http.get_current_application_info()?;
        Ok(Self {
            owner_id: info.owner.id,
            bot_id: info.id,
        })
    }

    pub fn configure<'a>(&self, cfg: &'a mut Configuration) -> &'a mut Configuration {
        let mut owners = HashSet::new();
        owners.insert(self.owner_id);
        cfg.on_mention(Some(self.bot_id)).owners(owners)
    }
}
