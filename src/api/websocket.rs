use axum::{
    extract::{Path, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    models::document::Document,
    services::{
        broadcast::{DocumentBroadcaster, DocumentUpdate},
        document::{get_document_with_permission, update_document},
    },
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Message {
    Update { content: String },
    Cursor { position: usize },
    Error { message: String },
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(document_id): Path<Uuid>,
    Extension(state): Extension<Arc<AppState>>,
    auth: AuthUser,
) -> impl IntoResponse {
    if let Ok((document, _permission)) =
        get_document_with_permission(&state.pool, document_id, auth.user_id).await
    {
        let sender = state.broadcaster.get_or_create_channel(document_id).await;
        ws.on_upgrade(move |socket| handle_socket(socket, document, auth, state, sender))
    } else {
        ws.on_upgrade(|socket| async {
            let (mut sender, _) = socket.split();
            let _ = sender
                .send(axum::extract::ws::Message::Text(
                    serde_json::to_string(&Message::Error {
                        message: "No permission".to_string(),
                    })
                    .unwrap(),
                ))
                .await;
        })
    }
}

async fn handle_socket(
    socket: axum::extract::ws::WebSocket,
    document: Document,
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    broadcast_sender: broadcast::Sender<DocumentUpdate>,
) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let mut broadcast_receiver = broadcast_sender.subscribe();

    // 创建一个任务来处理广播消息
    let send_task = tokio::spawn(async move {
        while let Ok(update) = broadcast_receiver.recv().await {
            // 不要发送自己的更新回自己
            if update.user_id != auth.user_id {
                let msg = serde_json::to_string(&Message::Update {
                    content: update.content,
                })
                .unwrap();
                if ws_sender
                    .send(axum::extract::ws::Message::Text(msg))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    });

    // 处理接收到的消息
    while let Some(Ok(message)) = ws_receiver.next().await {
        if let axum::extract::ws::Message::Text(text) = message {
            if let Ok(msg) = serde_json::from_str::<Message>(&text) {
                match msg {
                    Message::Update { content } => {
                        // 更新文档
                        if let Ok(_) =
                            update_document(&state.pool, document.id, None, Some(content.clone()))
                                .await
                        {
                            // 广播更新到其他连接
                            let update = DocumentUpdate {
                                document_id: document.id,
                                user_id: auth.user_id,
                                content,
                                cursor_position: None,
                            };
                            let _ = broadcast_sender.send(update);
                        }
                    }
                    Message::Cursor { position } => {
                        let update = DocumentUpdate {
                            document_id: document.id,
                            user_id: auth.user_id,
                            content: String::new(),
                            cursor_position: Some(position),
                        };
                        let _ = broadcast_sender.send(update);
                    }
                    _ => {}
                }
            }
        }
    }

    // 清理
    send_task.abort();
}
