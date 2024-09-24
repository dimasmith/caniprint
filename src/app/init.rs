use crate::subscriptions::subscribers::FileStorage;
use crate::subscriptions::Subscribers;
use crate::{Notifier, SubscriptionService};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn subscriptions_service(
    repo: Subscribers<FileStorage>,
    notifier: Box<dyn Notifier>,
) -> Arc<Mutex<SubscriptionService>> {
    let service = SubscriptionService::new(repo, notifier);
    Arc::new(Mutex::new(service))
}
