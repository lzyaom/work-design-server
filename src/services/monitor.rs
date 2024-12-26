use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, DiskExt, System, SystemExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub cpu_usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub disk_total: u64,
    pub disk_used: u64,
    pub process_count: usize,
}

pub fn get_system_status() -> SystemStatus {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

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
        process_count: sys.processes().len(),
    }
}
