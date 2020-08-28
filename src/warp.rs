use warp::{Filter, reject::Reject, Rejection, Reply};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize)]
struct PublishResponse {
    /// Is the request successful?
    success: bool,
}

#[derive(Debug)]
struct RequestFailed;
impl Reject for RequestFailed {}

async fn error_handler(_rejection: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
    let response = PublishResponse {
        success: false,
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        code
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
        
            let response = PublishResponse {
                success: true,
            };

            if let Ok(_) = request {
                Ok(warp::reply::json(&response))
            } else {
                Err(warp::reject::custom(RequestFailed))
            }
        }).recover(error_handler);

    warp::serve(publish).run(([127, 0, 0, 1], 3030)).await;
}
