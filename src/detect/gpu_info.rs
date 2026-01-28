use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GpuDetails {
    pub name: String,
    pub vendor: String,
    pub is_integrated: bool,
    pub shared_memory: bool,
}

pub fn detect_gpu() -> Option<GpuDetails> {
    // This remains informational as requested
    crate::gpu::get_active_backend_name().map(|name| {
        GpuDetails {
            name,
            vendor: "Detected Vendor".to_string(), // Simplified for now
            is_integrated: true, // Detection logic can be expanded
            shared_memory: true,
        }
    })
}
