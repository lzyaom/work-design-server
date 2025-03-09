use crate::models::message::SocketPushMessage;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct MessageBroadcast {
    channel: Arc<RwLock<Sender<SocketPushMessage>>>,
}

impl MessageBroadcast {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel::<SocketPushMessage>(capacity);
        Self {
            channel: Arc::new(RwLock::new(tx)),
        }
    }

    pub async fn subscribe(&self) -> Receiver<SocketPushMessage> {
        let sender = self.channel.read().await;
        sender.subscribe()
    }

    pub async fn publish(
        &self,
        msg: SocketPushMessage,
    ) -> Result<usize, broadcast::error::SendError<SocketPushMessage>> {
        let sender = self.channel.read().await;
        sender.send(msg)
    }
}
