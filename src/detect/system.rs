use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum PowerSource {
    AC,
    Battery,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemContext {
    pub power_source: PowerSource,
    pub battery_level: Option<f32>,
    pub is_low_power_mode: bool,
}

pub fn detect_context() -> SystemContext {
    // Basic detection for now, could be expanded with battery crates later
    SystemContext {
        power_source: PowerSource::Unknown,
        battery_level: None,
        is_low_power_mode: false,
    }
}
