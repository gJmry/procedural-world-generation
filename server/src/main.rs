mod models;
mod generators;
mod api;

#[tokio::main]
async fn main() {
    api::main::main().await;
}
