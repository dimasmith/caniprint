use caniprint::scheduler::init::start_scheduler;
use caniprint::subscriptions::Subscribers;
use caniprint::telegram::bot::start_bot;
use caniprint::SubscriptionService;
use std::error::Error;
use std::sync::Arc;
use teloxide::Bot;
use tokio::sync::Mutex;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let bot = Bot::from_env();
    let service_bot = bot.clone();

    let subscribers = Subscribers::from_file()?;
    let subscription_service = SubscriptionService::new(subscribers, service_bot);
    let shared_subscription_service = Arc::new(Mutex::new(subscription_service));

    let scheduled_subscription_service = Arc::clone(&shared_subscription_service);
    start_scheduler(scheduled_subscription_service).await?;

    let bot_subscription_service = Arc::clone(&shared_subscription_service);
    start_bot(bot, bot_subscription_service).await;

    Ok(())
}


