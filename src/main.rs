use caniprint::init::subscriptions_service;
use caniprint::scheduler::init::start_scheduler;
use caniprint::subscriptions::Subscribers;
use caniprint::telegram::bot::start_bot;
use caniprint::telegram::digest::BotDigestNotifier;
use std::error::Error;
use std::sync::Arc;
use teloxide::Bot;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let bot = Bot::from_env();
    let service_bot = bot.clone();

    let bot_notifier = Box::new(BotDigestNotifier::new(service_bot));
    let subscribers_repo = Subscribers::from_file()?;
    let subscription_service = subscriptions_service(subscribers_repo, bot_notifier);

    start_scheduler(Arc::clone(&subscription_service)).await?;
    start_bot(bot, Arc::clone(&subscription_service)).await;

    Ok(())
}
