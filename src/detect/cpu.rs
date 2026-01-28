use sysinfo::{System, CpuRefreshKind, RefreshKind};
use crate::cpu::arch::{detect_arch, CpuArch};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CpuDetails {
    pub arch: CpuArch,
    pub physical_cores: usize,
    pub logical_threads: usize,
    pub brand: String,
    pub frequency_mhz: u64,
    pub usage: f32,
}

pub fn detect_cpu() -> CpuDetails {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
    );
    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_millis(100));
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    let brand = if !cpus.is_empty() {
        cpus[0].brand().to_string()
    } else {
        "Unknown".to_string()
    };

    let usage = if !cpus.is_empty() {
        cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32
    } else {
        0.0
    };

    CpuDetails {
        arch: detect_arch(),
        physical_cores: System::physical_core_count().unwrap_or(0),
        logical_threads: cpus.len(),
        brand,
        frequency_mhz: if !cpus.is_empty() { cpus[0].frequency() } else { 0 },
        usage,
    }
}
