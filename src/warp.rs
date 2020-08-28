use warp::{Filter, reject::Reject};
use serde::{Deserialize, Serialize};
use crate::{publish, teloxide::create_bot, config::Config, CONFIG_FILENAME};

#[derive(Deserialize, Serialize)]
/// The request to /publish.
/// ```
/// GET /publish?message="Message"
/// ```
struct PublishRequest {
    /// The message to publish.
    message: String,
}

#[derive(Debug)]
/// When the request failed.
struct RequestFailed;
impl Reject for RequestFailed {}

pub async fn warp_server() {

 let publish = warp::path!("publish")
        .and(warp::query::<PublishRequest>())
        .and_then(|query: PublishRequest| async move {
            let config = Config::from_file(CONFIG_FILENAME);

            let request = publish::publish(
                &create_bot(),
                publish::MessageRequest::new(query.message),
                config.telexide.publish_channel,
            ).await;
        
            match request {
                Ok(_) => { Ok(r#"{"success": true}"#) },
                Err(_) => { Err(warp::reject::custom(RequestFailed)) },
            }
        });

    warp::serve(publish).run(([127, 0, 0, 1], 3030)).await;
}
