use caniprint::subscriptions::Subscribers;
use caniprint::telegram::bot::start_bot;
use caniprint::SubscriptionService;
use chrono::Local;
use std::error::Error;
use std::sync::Arc;
use teloxide::Bot;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::{info, warn};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let bot = Bot::from_env();
    let subscribers = Subscribers::from_file()?;

    let service_bot = bot.clone();
    let subscription_service = SubscriptionService::new(subscribers, service_bot);
    let shared_subscription_service = Arc::new(Mutex::new(subscription_service));

    let scheduler = JobScheduler::new().await?;

    let scheduled_subscription_service = Arc::clone(&shared_subscription_service);
    let digest_job = create_digest_job(scheduled_subscription_service)?;
    scheduler.add(digest_job).await?;
    info!("Digest job initialized");
    scheduler.start().await?;

    let bot_subscription_service = Arc::clone(&shared_subscription_service);
    start_bot(bot, bot_subscription_service).await;

    Ok(())
}

fn create_digest_job(
    subscription_service: Arc<Mutex<SubscriptionService>>,
) -> Result<Job, JobSchedulerError> {
    Job::new_async_tz("0 0 9 * * *", Local, move |_uuid, _l| {
        let service = Arc::clone(&subscription_service);
        Box::pin(async move {
            let service = service.lock().await;
            service
                .send_digests_to_subscribers()
                .await
                .map_err(|e| warn!("Failed to send digests: {}", e))
                .ok();
        })
    })
}
