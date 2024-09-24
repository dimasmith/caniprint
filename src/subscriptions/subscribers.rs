use directories::ProjectDirs;
use std::collections::HashSet;
use std::path::PathBuf;
use thiserror::Error;
use tracing::{debug, error, info, instrument};
use crate::SubscriberId;

#[derive(Debug, Error)]
pub enum SubscribersError {
    #[error("IO error: {0}")]
    IoError(std::io::Error),
}

pub type SubscribersRepository = Subscribers<FileStorage>;

#[derive(Debug)]
pub struct Subscribers<Storage> {
    storage: Storage,
    subscribers: HashSet<SubscriberId>,
}

impl Subscribers<FileStorage> {
    pub fn from_file() -> Result<Self, SubscribersError> {
        let storage = FileStorage::new();
        let subscribers = storage.read().map_err(SubscribersError::IoError)?;
        Ok(Self {
            storage,
            subscribers,
        })
    }
}

impl Subscribers<FileStorage> {

    #[instrument]
    pub async fn subscribe(&mut self, chat_id: SubscriberId) {
        self.subscribers.insert(chat_id);
        if let Err(e) = self.storage.write(&self.subscribers) {
            error!("failed to save subscribers: {}", e);
        }
    }

    #[instrument]
    pub async fn unsubscribe(&mut self, chat_id: SubscriberId) {
        self.subscribers.remove(&chat_id);
        if let Err(e) = self.storage.write(&self.subscribers) {
            error!("failed to save subscribers: {}", e);
        }
    }

    pub async fn subscribers(&self) -> Vec<SubscriberId> {
        self.subscribers.iter().cloned().collect()
    }
}

#[derive(Debug)]
pub struct FileStorage {
    path: PathBuf,
}

impl FileStorage {
    fn new() -> Self {
        let project_dirs = ProjectDirs::from("dev", "anatolich", "caniprintbot").unwrap();
        let data_dir = project_dirs.data_dir();
        if !data_dir.exists() {
            std::fs::create_dir_all(data_dir).unwrap();
        }
        let file_path = data_dir.join("subscribers.dat");
        if !file_path.exists() {
            std::fs::write(&file_path, "").unwrap();
            debug!("Created a new subscribers file at {:?}", file_path);
        }
        Self { path: file_path }
    }

    fn read(&self) -> Result<HashSet<SubscriberId>, std::io::Error> {
        let content = std::fs::read_to_string(&self.path)?;
        let mut subscribers = HashSet::new();
        for line in content.lines() {
            if let Ok(chat_id) = line.parse::<i64>() {
                subscribers.insert(SubscriberId(chat_id));
            }
        }
        info!("subscribers loaded from {}", self.path.display());
        Ok(subscribers)
    }

    fn write(&self, subscribers: &HashSet<SubscriberId>) -> Result<(), std::io::Error> {
        let content = subscribers
            .iter()
            .map(|subscriber_id| subscriber_id.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let result = std::fs::write(&self.path, content);
        info!("subscribers saved to {}", self.path.display());
        result
    }
}

impl SubscriberId {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}