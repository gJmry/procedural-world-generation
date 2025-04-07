mod models;
mod generators;
mod api;

#[tokio::main]
async fn main() {
    let _ = api::main::main().await;
}
