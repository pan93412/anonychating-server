use warp::{Filter, reject::Reject, Rejection, Reply};
use serde::Deserialize;
use crate::{publish, teloxide::create_bot, config::Config, CONFIG_FILENAME};

#[derive(Deserialize)]
/// The request to /publish.
/// ```
/// GET /publish?message="Message"
/// ```
struct PublishRequest {
    /// The message to publish.
    message: String,
}

#[derive(Debug)]
struct RequestFailed;
impl Reject for RequestFailed {}

async fn error_handler(_rejection: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    Ok(warp::reply::with_status(
        r#"{"success": false}"#,
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

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

            if let Ok(_) = request {
                Ok(r#"{"success": true}"#)
            } else {
                Err(warp::reject::custom(RequestFailed))
            }
        }).recover(error_handler);

    warp::serve(publish).run(([127, 0, 0, 1], 3030)).await;
}
