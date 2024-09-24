use crate::blackout::Digest;
use crate::load_forecast_digest;
use crate::subscriptions::subscribers::SubscribersRepository;
use async_trait::async_trait;
use futures::future::join_all;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SubscriberId(pub i64);

#[derive(Debug, Error)]
pub enum SubscriptionError {
    #[error("Failed to retrieve digest")]
    DigestRetrievalError,
}

#[derive(Debug)]
pub struct SubscriptionService {
    subscribers: SubscribersRepository,
    notifier: Box<dyn Notifier>,
}

impl SubscriptionService {
    pub fn new(subscribers: SubscribersRepository, notifier: Box<dyn Notifier>) -> Self {
        Self {
            subscribers,
            notifier,
        }
    }
}

impl SubscriptionService {
    pub async fn subscribe(&mut self, chat_id: SubscriberId) {
        self.subscribers.subscribe(chat_id).await;
    }

    pub async fn send_digests_to_subscribers(&self) -> Result<(), SubscriptionError> {
        let clients = self.subscribers.subscribers().await;
        let digest = load_forecast_digest(3).await;
        let notifier = &self.notifier;
        match digest {
            Ok(d) => {
                let notifications: Vec<_> = clients
                    .iter()
                    .map(|client| notifier.send_digest(*client, &d))
                    .collect();
                join_all(notifications).await;
            }
            Err(_) => {
                let notifications: Vec<_> = clients
                    .iter()
                    .map(|client| notifier.send_digest_unavailable(*client))
                    .collect();
                join_all(notifications).await;
            }
        }
        Ok(())
    }
}

impl From<i64> for SubscriberId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

#[async_trait]
pub trait Notifier: Send + Sync + Debug {
    async fn send_digest(
        &self,
        chat_id: SubscriberId,
        message: &Digest,
    ) -> Result<(), SubscriptionError>;

    async fn send_digest_unavailable(&self, chat_id: SubscriberId)
        -> Result<(), SubscriptionError>;
}

#[cfg(test)]
mod tests {

    use super::*;

    fn is_normal<T: Sized + Send + Sync + Unpin>() {}
    #[test]
    fn can_pass_subscription_service() {
        is_normal::<SubscriptionService>();
    }
}
