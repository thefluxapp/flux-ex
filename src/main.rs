mod app;

#[tokio::main]
async fn main() {
    app::main().await;
}
