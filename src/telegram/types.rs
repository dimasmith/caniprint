use crate::SubscriberId;
use teloxide::types::ChatId;

impl From<ChatId> for SubscriberId {
    fn from(id: ChatId) -> Self {
        SubscriberId(id.0)
    }
}

impl From<SubscriberId> for ChatId {
    fn from(id: SubscriberId) -> Self {
        ChatId(id.0)
    }
}
