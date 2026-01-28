use serde::Serialize;
use crate::hardware::{CpuInfo, GpuInfo};

#[derive(Debug, Clone, Serialize)]
pub struct HardwareState {
    pub cpu: CpuInfo,
    pub gpu: Option<GpuInfo>,
    pub available_memory_gb: f64,
    pub timestamp: u64,
}

pub trait HardwareProvider: Send + Sync {
    fn capture_state(&self) -> HardwareState;
}

pub struct DefaultHardwareProvider;

impl HardwareProvider for DefaultHardwareProvider {
    fn capture_state(&self) -> HardwareState {
        let info = crate::hardware::SystemInfo::detect();
        HardwareState {
            cpu: info.cpu,
            gpu: info.gpu,
            available_memory_gb: info.available_memory_gb,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}
