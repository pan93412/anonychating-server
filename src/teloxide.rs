use teloxide::prelude::*;

pub fn create_bot() -> teloxide::Bot {
    teloxide::enable_logging!();
    let bot = Bot::from_env();
    
    bot
}
