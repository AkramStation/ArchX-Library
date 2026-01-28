use sysinfo::{System, MemoryRefreshKind, RefreshKind};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MemoryDetails {
    pub total_kb: u64,
    pub available_kb: u64,
    pub used_kb: u64,
    pub swap_total_kb: u64,
    pub swap_free_kb: u64,
    pub pressure_estimate: f32, // 0.0 to 1.0
}

pub fn detect_memory() -> MemoryDetails {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything())
    );
    sys.refresh_memory();

    let total = sys.total_memory();
    let available = sys.available_memory();
    let used = sys.used_memory();
    
    let pressure = if total > 0 {
        used as f32 / total as f32
    } else {
        0.0
    };

    MemoryDetails {
        total_kb: total,
        available_kb: available,
        used_kb: used,
        swap_total_kb: sys.total_swap(),
        swap_free_kb: sys.free_swap(),
        pressure_estimate: pressure,
    }
}
