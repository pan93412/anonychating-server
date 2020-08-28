#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    anonychating_server::warp::warp_server().await;
}
