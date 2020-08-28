use warp::{Filter, reject::Reject};
use serde::{Deserialize, Serialize};
use crate::{publish, teloxide::create_bot};

#[derive(Deserialize, Serialize)]
struct PublishRequest {
    message: String,
}

#[derive(Debug)]
struct RequestFailed;
impl Reject for RequestFailed {}

pub async fn warp_server() {
    let hello = warp::path!("publish")
        .and(warp::query::<PublishRequest>())
        .and_then(|query: PublishRequest| async move {
            let request = publish::publish(
                &create_bot(),
                publish::MessageRequest::new(query.message)
            ).await;
        
            match request {
                Ok(_) => { Ok(r#"{"success": true}"#) },
                Err(_) => { Err(warp::reject::custom(RequestFailed)) },
            }
        });

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
