use crate::{config::Config, log::log_info, publish, teloxide::create_bot, CONFIG_FILENAME};
use serde::Deserialize;
use warp::{reject::Reject, Filter, Rejection, Reply};

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
    let wc = Config::from_file(CONFIG_FILENAME).warp;
    let publish = warp::path!("publish")
        .and(warp::query::<PublishRequest>())
        .and_then(|query: PublishRequest| async move {
            let config = Config::from_file(CONFIG_FILENAME);
            let bot = create_bot().await;

            let request = publish::publish(
                &bot,
                publish::MessageRequest::new(query.message),
                config.telexide.publish_channel,
            );

            if request.await.is_ok() {
                Ok(r#"{"success": true}"#)
            } else {
                Err(warp::reject::custom(RequestFailed))
            }
        })
        .recover(error_handler);

    // wc.server_ip
    //  1. 先變成迭代器，以便 map 成字串: iter()
    //  2. 將每個內部的數字變成字串: map(|v| v.to_string())
    //  3. 然後，重新組回一個 Vector (顯式宣告): .collect::<Vec<String>>()
    //  4. 最後，中間加點點: .join(".")
    log_info(
        "Warp.Main",
        &format!(
            "Server is running on \x1b[1mhttp://{}\x1b[0m",
            wc.to_uri(),
        ),
    );
    warp::serve(publish)
        .run((wc.server_ip, wc.server_port))
        .await;
}
