use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sysinfo::{CpuExt, DiskExt, NetworkExt, System, SystemExt};
use tokio::sync::{broadcast, RwLock};
use tokio::time::{interval, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub cpu_usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub disk_total: u64,
    pub disk_used: u64,
    pub network_up: u64,
    pub network_down: u64,
    pub process_count: usize,
    pub timestamp: i64,
}

pub struct SystemStatusBroadcaster {
    channel: Arc<RwLock<broadcast::Sender<SystemStatus>>>,
}

impl SystemStatusBroadcaster {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self {
            channel: Arc::new(RwLock::new(sender)),
        }
    }

    pub async fn get_channel(&self) -> broadcast::Sender<SystemStatus> {
        self.channel.read().await.clone()
    }

    pub async fn start_broadcast_task(self: Arc<Self>) {
        // 创建一个定时任务，每 5 秒发送一次系统状态
        let mut interval = interval(Duration::from_secs(5));
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                let status = get_system_status();
                if let Ok(sender) = self.channel.try_read() {
                    // 发送系统状态
                    let _ = sender.send(status);
                }
            }
        });
    }
}

pub fn get_system_status() -> SystemStatus {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
    let mut network_up = 0;
    let mut network_down = 0;
    for (_, network) in sys.networks() {
        network_up += network.transmitted();
        network_down += network.received();
    }
    SystemStatus {
        cpu_usage,
        memory_total: sys.total_memory(),
        memory_used: sys.used_memory(),
        disk_total: sys.disks().iter().map(|disk| disk.total_space()).sum(),
        disk_used: sys
            .disks()
            .iter()
            .map(|disk| disk.total_space() - disk.available_space())
            .sum(),
        network_down,
        network_up,
        process_count: sys.processes().len(),
        timestamp: chrono::Utc::now().timestamp(),
    }
}
