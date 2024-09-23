use directories::ProjectDirs;
use std::collections::HashSet;
use std::path::PathBuf;
use teloxide::types::ChatId;

pub struct Subscribers<Storage> {
    storage: Storage,
    subscribers: HashSet<ChatId>,
}

impl Subscribers<FileStorage> {
    pub fn from_file() -> Self {
        let storage = FileStorage::new();
        let subscribers = storage.read().unwrap();
        Self {
            storage,
            subscribers,
        }
    }
}

impl Subscribers<FileStorage> {
    pub async fn subscribe(&mut self, chat_id: ChatId) {
        self.subscribers.insert(chat_id);
        self.storage.write(&self.subscribers).unwrap();
    }

    pub async fn unsubscribe(&mut self, chat_id: ChatId) {
        self.subscribers.remove(&chat_id);
        self.storage.write(&self.subscribers).unwrap();
    }

    pub async fn subscribers(&self) -> Vec<ChatId> {
        self.subscribers.iter().cloned().collect()
    }
}

impl Subscribers<MemoryStorage> {
    pub fn from_memory() -> Self {
        Self {
            storage: MemoryStorage {},
            subscribers: HashSet::new(),
        }
    }

    pub async fn subscribe(&mut self, chat_id: ChatId) {
        self.subscribers.insert(chat_id);
    }

    pub async fn unsubscribe(&mut self, chat_id: ChatId) {
        self.subscribers.remove(&chat_id);    }

    pub async fn subscribers(&self) -> Vec<ChatId> {
        self.subscribers.iter().cloned().collect()
    }
}

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
        }
        Self { path: file_path }
    }

    fn read(&self) -> Result<HashSet<ChatId>, std::io::Error> {
        let content = std::fs::read_to_string(&self.path)?;
        let mut subscribers = HashSet::new();
        for line in content.lines() {
            if let Ok(chat_id) = line.parse::<i64>() {
                subscribers.insert(ChatId(chat_id));
            }
        }
        Ok(subscribers)
    }

    fn write(&self, subscribers: &HashSet<ChatId>) -> Result<(), std::io::Error> {
        let content = subscribers
            .iter()
            .map(|chat_id| chat_id.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        std::fs::write(&self.path, content)
    }
}

pub struct MemoryStorage {

}
