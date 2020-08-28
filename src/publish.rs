use teloxide::{Bot, prelude::{Request, Message}, RequestError};

pub struct MessageRequest {
    message_text: String,
}

impl MessageRequest {
    /// Constructs a Message.
    pub fn new(message_text: String) -> Self {
        MessageRequest { message_text }
    }
}

/// Publish message to @anonychating.
pub async fn publish(bot: &Bot, message: MessageRequest, publish_to: String) -> Result<Message, RequestError> {
    let request = bot.send_message(
        publish_to,
        message.message_text,
    );

    Ok(request.send().await?)
}
