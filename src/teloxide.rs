use crate::{config::Config, CONFIG_FILENAME};
use teloxide::{BotBuilder};

pub async fn create_bot() -> teloxide::Bot {
    let config = Config::from_file(CONFIG_FILENAME);
    BotBuilder::new().token(config.telexide.bot_token).build()
    // let me = bot.get_me().send().await.expect("The bot token should be invaild.");
    // log_info("Telexide.CreateBot", &format!(
    //     "Created a bot instance to @{}. It forwards messages to: {}",
    //     me.user.username.expect("Bots should have username"),
    //     config.telexide.publish_channel,
    // ));
}
