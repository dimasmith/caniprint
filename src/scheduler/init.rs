use crate::scheduler::jobs::send_digest_job;
use crate::SubscriptionService;
use chrono::Local;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{JobBuilder, JobScheduler, JobSchedulerError, JobToRunAsync};

pub async fn start_scheduler(
    subscriptions_service: Arc<Mutex<SubscriptionService>>,
) -> Result<(), JobSchedulerError> {
    let scheduler = JobScheduler::new().await?;
    let send_digest_job = send_digest_job(Arc::clone(&subscriptions_service));

    schedule_job("0 0 9 * * *", send_digest_job, &scheduler).await?;

    scheduler.start().await
}

async fn schedule_job(
    schedule: &str,
    job: Box<JobToRunAsync>,
    scheduler: &JobScheduler,
) -> Result<(), JobSchedulerError> {
    let job_config = JobBuilder::new()
        .with_timezone(Local)
        .with_cron_job_type()
        .with_run_async(job)
        .with_schedule(schedule)?
        .build()?;
    scheduler.add(job_config).await?;
    Ok(())
}
