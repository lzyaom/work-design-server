use crate::{
    api::AppState,
    error::AppError,
    middleware::auth::AuthUser,
    models::user::UserRole,
    services::monitor::{get_system_status, SystemStatus},
};
use axum::{
    extract::{ws, Extension, WebSocketUpgrade},
    response::IntoResponse,
    Json,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn get_status(
    auth: AuthUser,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<SystemStatus>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let status = get_system_status();
    Ok(Json(status))
}

pub async fn ws_monitor(
    ws: WebSocketUpgrade,
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return ws.on_upgrade(|socket| async {
            let (mut sender, _) = socket.split();
            let _ = sender
                .send(ws::Message::Text("Unauthorized".to_string()))
                .await;
        });
    }

    // 建立 WebSocket 连接
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: ws::WebSocket, state: Arc<AppState>) {
    // 拆分 WebSocket
    let (mut sender, mut receiver) = socket.split();

    // 创建内部通信通道（用于接收线程向发送线程传递消息）
    let (tx, mut rx) = mpsc::channel(10);

    // 订阅系统状态更新
    let channel = state.monitor_broadcaster.get_channel().await;
    let mut monitor_rx = channel.subscribe();

    // 创建一个发送任务来处理系统状态更新， 并发送给客户端
    let mut send_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                // 监听系统状态广播
                Ok(metrics) = monitor_rx.recv() => {
                    if let Ok(msg) = serde_json::to_string(&metrics) {
                        if let Err(e) = sender.send(ws::Message::Text(msg)).await {
                            tracing::error!("发送监控数据失败: {}", e);
                            break;
                        }
                    }
                },
                // 监听接收任务的消息
                Some(msg) = rx.recv() => {
                    if let Err(e) = sender.send(msg).await{
                        // 发送响应消息失败
                        tracing::error!("发送响应信息失败: {}", e);
                        break;
                    }
                },
            }
        }
    });

    // 创建一个接收任务来处理客户端消息, 例如 ping/pong
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // 处理客户端消息 (例如： ping/pong)
            match msg {
                ws::Message::Ping(ping) => {
                    // 通过通道转发 Pong 消息
                    let _ = tx.send(ws::Message::Pong(ping)).await;
                }
                ws::Message::Close(_) => break, // 关闭连接
                _ => {}                         // 忽略其他消息
            }
        }
    });

    // 等待任务完成
    let _ = tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
