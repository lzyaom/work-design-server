use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentUpdate {
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub cursor_position: Option<usize>,
}

pub struct DocumentBroadcaster {
    channels: Arc<RwLock<HashMap<Uuid, Sender<DocumentUpdate>>>>,
}

impl DocumentBroadcaster {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_or_create_channel(
        &self,
        document_id: Uuid,
    ) -> broadcast::Sender<DocumentUpdate> {
        let mut channels = self.channels.write().await;
        if let Some(sender) = channels.get(&document_id) {
            sender.clone()
        } else {
            let (sender, _) = broadcast::channel(100);
            channels.insert(document_id, sender.clone());
            sender
        }
    }

    pub async fn remove_channel(&self, document_id: Uuid) {
        let mut channels = self.channels.write().await;
        channels.remove(&document_id);
    }
}
