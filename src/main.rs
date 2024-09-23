use caniprint::subscriptions::subscribers::FileStorage;
use caniprint::subscriptions::Subscribers;
use caniprint::telegram::bot::{send_forecast_digest, start_bot};
use caniprint::load_forecast_digest;
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
    let subscribers = Arc::new(Mutex::new(subscribers));
    let scheduler = JobScheduler::new().await?;

    let digest_bot = bot.clone();
    let digest_subscribers = Arc::clone(&subscribers);
    let digest_job = create_digest_job(digest_bot, digest_subscribers)?;
    scheduler.add(digest_job).await?;
    info!("Digest job initialized");
    scheduler.start().await?;

    start_bot(bot, subscribers).await;

    Ok(())
}

fn create_digest_job(
    bot: Bot,
    subscribers: Arc<Mutex<Subscribers<FileStorage>>>,
) -> Result<Job, JobSchedulerError> {
    Job::new_async("0 0 9 * * *", move |_uuid, _l| {
        let digest_bot = bot.clone();
        let digest_subscribers = Arc::clone(&subscribers);
        Box::pin(async move {
            let clients = digest_subscribers.lock().await.subscribers().await;
            let digest = load_forecast_digest(3).await;
            match digest {
                Ok(d) => {
                    if let Err(e) = send_forecast_digest(digest_bot, &clients, &d).await {
                        warn!("Failed to send digest: {}", e);
                    }
                }
                Err(e) => {
                    warn!("Failed to load digest: {}", e);
                }
            }
        })
    })
}
