use async_trait::async_trait;
use teloxide::Bot;
use crate::{Notifier, SubscriberId, SubscriptionError};
use crate::blackout::Digest;
use crate::telegram::messages::{send_digest, send_digest_unavailable};

#[derive(Debug)]
pub struct BotDigestNotifier {
    bot: Bot,
}

impl BotDigestNotifier {
    pub fn new(bot: Bot) -> Self {
        Self { bot }
    }
}

#[async_trait]
impl Notifier for BotDigestNotifier {
    async fn send_digest(
        &self,
        chat_id: SubscriberId,
        digest: &Digest,
    ) -> Result<(), SubscriptionError> {
        let bot = &self.bot;
        send_digest(bot, chat_id, digest)
            .await
            .map_err(|_| SubscriptionError::DigestRetrievalError)?;
        Ok(())
    }

    async fn send_digest_unavailable(
        &self,
        chat_id: SubscriberId,
    ) -> Result<(), SubscriptionError> {
        let bot = &self.bot;
        send_digest_unavailable(bot, chat_id)
            .await
            .map_err(|_| SubscriptionError::DigestRetrievalError)?;
        Ok(())
    }
}