use crate::system;
use crate::optimizer::gpu;

/// A diagnostic structure that is easy to serialize for Tauri/UI apps.
/// 
/// WHY: Tauri apps communicate via JSON. Providing a dedicated structure 
/// ensures seamless integration without manual conversion logic.
#[derive(Debug, serde::Serialize)]
pub struct SystemDiagnostics {
    pub arch: String,
    pub bits: String,
    pub cpu_features: Vec<String>,
    pub gpu_backend: Option<String>,
    pub thread_count: usize,
}

/// A Tauri-compatible command to diagnose the system.
/// 
/// NOTE: This is intended to be used inside a Tauri `#[tauri::command]`.
pub fn diagnose_system() -> SystemDiagnostics {
    let info = system::get_info();
    let mut features = Vec::new();
    if info.features.sse2 { features.push("SSE2".to_string()); }
    if info.features.avx { features.push("AVX".to_string()); }
    if info.features.avx2 { features.push("AVX2".to_string()); }
    if info.features.avx512f { features.push("AVX512F".to_string()); }

    SystemDiagnostics {
        arch: format!("{:?}", info.arch),
        bits: format!("{:?}", info.bits),
        cpu_features: features,
        gpu_backend: gpu::get_active_backend_name(),
        thread_count: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
    }
}
