use caniprint::telegram::bot::init_bot;

#[tokio::main]
async fn main() {
    init_bot().await;
}
