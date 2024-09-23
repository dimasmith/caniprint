use std::collections::HashSet;
use teloxide::types::ChatId;

pub struct Subscribers {
    subscribers: HashSet<ChatId>
}

impl Subscribers {
    pub fn new() -> Self {
        Self {
            subscribers: HashSet::new()
        }
    }
}

impl Subscribers {
    pub async fn subscribe(&mut self, chat_id: ChatId) {
        self.subscribers.insert(chat_id);
    }

    pub async fn unsubscribe(&mut self, chat_id: ChatId) {
        self.subscribers.remove(&chat_id);
    }

    pub async fn subscribers(&self) -> Vec<ChatId> {
        self.subscribers.iter().cloned().collect()
    }
}