use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum DeviceType {
    Cpu,
    IntegratedGpu,
    DedicatedGpu,
    WasmSafe,
}

#[derive(Debug, Clone, Serialize)]
pub struct Device {
    pub name: String,
    pub device_type: DeviceType,
    pub vendor: String,
    pub memory_shared: bool,
    pub compute_available: bool,
}
