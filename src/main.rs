#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    rustblog::warp::warp_server().await;
}
