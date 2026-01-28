use crate::detect::HardwareState;

pub struct LoadMonitor;

impl LoadMonitor {
    pub fn is_saturated(state: &HardwareState) -> bool {
        state.cpu.usage > 0.95
    }

    pub fn is_under_pressure(state: &HardwareState) -> bool {
        state.memory.pressure_estimate > 0.9
    }
}
