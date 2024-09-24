use crate::SubscriptionService;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::JobToRunAsync;
use tracing::warn;

pub fn send_digest_job(
    subscription_service: Arc<Mutex<SubscriptionService>>,
) -> Box<JobToRunAsync> {
    Box::new(move |_uuid, _l| {
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
