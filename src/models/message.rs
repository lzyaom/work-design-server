use serde::{Deserialize, Serialize};

use super::document::DocumentUpdateMessage;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum SocketPushMessage {
    SystemMetrics(),
    TaskProgress(),
    Notification(),
    Document(DocumentUpdateMessage),
}

/// 客户端可订阅的消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    /// 系统监控数据 (CPU/内存/网络等)
    System,
    /// 任务执行进度
    Task,
    /// 系统通知消息
    Notification,
    /// 审计日志更新
    AuditLog,
    /// 实时协同编辑事件
    Collaboration,
    /// 调试信息（仅管理员可见）
    Debug,
    /// 文档更新
    Document,
}

/// 客户端控制命令
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ClientCommand {
    Subscribe(MessageType),
    Unsubscribe(MessageType),
    /// 带过滤条件的订阅（示例）
    SubscribeWithFilter {
        #[serde(rename = "type")]
        msg_type: MessageType,
        filter: Option<String>,
    },
}
