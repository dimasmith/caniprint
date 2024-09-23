use caniprint::subscriptions::Subscribers;
use caniprint::telegram::bot::{send_forecast_digest, start_bot};
use caniprint::ztoe::service::digest_forecasts;
use std::error::Error;
use std::sync::Arc;
use teloxide::Bot;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot = Bot::from_env();
    let subscribers = Arc::new(Mutex::new(Subscribers::new()));
    let scheduler = JobScheduler::new().await?;

    let digest_bot = bot.clone();
    let digest_subscribers = Arc::clone(&subscribers);
    let digest_job = create_digest_job(digest_bot, digest_subscribers)?;

    scheduler.add(digest_job).await?;
    scheduler.start().await?;
    start_bot(bot, subscribers).await;

    Ok(())
}

fn create_digest_job(
    bot: Bot,
    subscribers: Arc<Mutex<Subscribers>>,
) -> Result<Job, JobSchedulerError> {
    Job::new_async("0 0 9 * * *", move |_uuid, _l| {
        let digest_bot = bot.clone();
        let digest_subscribers = Arc::clone(&subscribers);
        Box::pin(async move {
            let digest = digest_forecasts(3).await;
            send_forecast_digest(digest_bot, digest_subscribers, &digest)
                .await
                .unwrap();
        })
    })
}
