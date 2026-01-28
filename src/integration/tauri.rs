use crate::system;

/// A diagnostic structure that is easy to serialize for Tauri/UI apps in v2.0.0.
/// 
/// Updated to use the unified SystemInfo architecture.
#[derive(Debug, serde::Serialize)]
pub struct SystemDiagnostics {
    pub arch: String,
    pub bits: String,
    pub cpu_features: Vec<String>,
    pub gpu_info: Option<crate::hardware::GpuInfo>,
    pub available_memory_gb: f64,
    pub thread_count: usize,
}

/// A Tauri-compatible command to diagnose the system.
/// 
/// NOTE: This is intended to be used inside a Tauri `#[tauri::command]`.
pub fn diagnose_system() -> SystemDiagnostics {
    let info = system::get_system_info();
    let mut features = Vec::new();
    
    // Feature enumeration for v2.0
    if info.cpu.features.sse2 { features.push("SSE2".to_string()); }
    if info.cpu.features.avx { features.push("AVX".to_string()); }
    if info.cpu.features.avx2 { features.push("AVX2".to_string()); }
    if info.cpu.features.avx512f { features.push("AVX512F".to_string()); }
    if info.cpu.features.neon { features.push("NEON".to_string()); }

    SystemDiagnostics {
        arch: format!("{:?}", info.cpu.arch),
        bits: format!("{:?}", info.cpu.bits),
        cpu_features: features,
        gpu_info: info.gpu,
        available_memory_gb: info.available_memory_gb,
        thread_count: info.cpu.logical_processors,
    }
}
