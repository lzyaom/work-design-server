use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    models::message::{ClientCommand, MessageType, SocketPushMessage},
    services::broadcast::MessageBroadcast,
};

// 处理 WebSocket 连接
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<AppState>>,
    auth: AuthUser,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.broadcaster.clone(), auth))
}

async fn handle_socket(socket: WebSocket, broadcast: Arc<MessageBroadcast>, auth: AuthUser) {
    let (mut sender, mut receiver) = socket.split();

    let mut broadcast_receiver = broadcast.subscribe().await;

    let subscriptions = Arc::new(Mutex::new(HashSet::new()));
    let subscriptions_clone = subscriptions.clone();

    // 创建一个任务来处理广播消息
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = broadcast_receiver.recv().await {
            if should_send(&subscriptions_clone.lock().unwrap(), &msg) {
                let msg = serde_json::to_string(&msg).unwrap();
                if let Err(e) = sender.send(Message::Text(msg)).await {
                    eprintln!("Error sending message: {}", e);
                    break;
                }
            }
        }
    });
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            {
                let mut subs = subscriptions.lock().unwrap();
                handle_client_message(&mut subs, &message, &auth);
            }
        }
    });

    // 等待两个任务完成
    let _ = tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    };
}

fn should_send(subscriptions: &HashSet<MessageType>, msg: &SocketPushMessage) -> bool {
    match msg {
        SocketPushMessage::Document(_) => subscriptions.contains(&MessageType::Document),
        SocketPushMessage::TaskProgress() => subscriptions.contains(&MessageType::Task),
        SocketPushMessage::Notification() => subscriptions.contains(&MessageType::Notification),
        SocketPushMessage::SystemMetrics() => subscriptions.contains(&MessageType::System),
    }
}

fn handle_client_message(
    subscriptions: &mut HashSet<MessageType>,
    message: &Message,
    auth: &AuthUser,
) {
    // 处理客户端发送的消息
    // 例如，解析消息类型和内容，更新订阅列表等
    if let Message::Text(text) = message {
        if let Ok(cmd) = serde_json::from_str::<ClientCommand>(&text) {
            match cmd {
                ClientCommand::Subscribe(msg_type) => {
                    subscriptions.insert(msg_type);
                }
                ClientCommand::Unsubscribe(msg_type) => {
                    subscriptions.remove(&msg_type);
                }
                ClientCommand::SubscribeWithFilter { msg_type, filter } => {
                    // 处理带过滤条件的订阅
                    // 例如，根据 filter 过滤消息并订阅
                    // 这里只是一个示例，具体实现取决于你的需求
                    if filter == Some("admin".to_string()) && auth.is_admin() {
                        subscriptions.insert(msg_type);
                    }
                }
            }
        }
    }
}
