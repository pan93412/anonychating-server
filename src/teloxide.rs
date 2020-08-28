use teloxide::prelude::*;

pub fn create_bot() -> teloxide::Bot {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();
    
    bot
}
