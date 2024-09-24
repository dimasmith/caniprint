use crate::load_forecast_digest;
use crate::subscriptions::subscribers::SubscribersRepository;
use crate::telegram::messages::{send_digest, send_digest_unavailable};
use futures::future::join_all;
use teloxide::types::ChatId;
use teloxide::Bot;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubscriptionError {
    #[error("Failed to retrieve digest")]
    DigestRetrievalError,
}

#[derive(Debug)]
pub struct SubscriptionService {
    subscribers: SubscribersRepository,
    bot: Bot,
}

impl SubscriptionService {
    pub fn new(subscribers: SubscribersRepository, bot: Bot) -> Self {
        Self { subscribers, bot }
    }
}

impl SubscriptionService {
    pub async fn subscribe(&mut self, chat_id: ChatId) {
        self.subscribers.subscribe(chat_id).await;
    }

    pub async fn send_digests_to_subscribers(&self) -> Result<(), SubscriptionError> {
        let clients = self.subscribers.subscribers().await;
        let digest = load_forecast_digest(3).await;
        let bot = &self.bot;
        match digest {
            Ok(d) => {
                let notifications: Vec<_> = clients
                    .iter()
                    .map(|client| send_digest(bot, *client, &d))
                    .collect();
                join_all(notifications).await;
            }
            Err(_) => {
                let notifications: Vec<_> = clients
                    .iter()
                    .map(|client| send_digest_unavailable(bot, *client))
                    .collect();
                join_all(notifications).await;
            }
        }
        Ok(())
    }
}
