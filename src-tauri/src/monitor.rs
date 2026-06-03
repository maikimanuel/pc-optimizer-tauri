use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub memory_total: u64,
    pub disk_free: u64,
    pub disk_total: u64,
}

pub fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        cpu_usage: 0.0,
        memory_usage: 0,
        memory_total: 0,
        disk_free: 0,
        disk_total: 0,
    })
}
