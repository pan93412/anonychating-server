use teloxide::{BotBuilder};
use crate::{config::Config, CONFIG_FILENAME};

pub fn create_bot() -> teloxide::Bot {
    let config = Config::from_file(CONFIG_FILENAME);
    BotBuilder::new().token(config.telexide.bot_token).build()
}
