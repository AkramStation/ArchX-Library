use archx::{add, add_advanced, WorkloadHints, register_backend, GpuBackend};
use archx::integration::tauri::diagnose_system;

// Imagine some Tauri types (mocked for the example)
mod tauri_mock {
    pub use serde::Serialize;
}

#[derive(tauri_mock::Serialize)]
struct CommandResponse {
    result: Vec<f32>,
    diagnostics: archx::integration::tauri::SystemDiagnostics,
}

/// A simulated Tauri command: archx_add
/// 
/// WHY: This shows how simple it is to wrap ArchX in a Tauri-compatible 
/// Rust command. It leverages the new 'diagnose_system' to return 
/// hardware context to the frontend.
fn archx_add(a: Vec<f32>, b: Vec<f32>) -> CommandResponse {
    let mut out = vec![0.0; a.len()];
    
    // Auto-dispatch handles CPU/GPU/Parallel logic
    add(&a, &b, &mut out);
    
    CommandResponse {
        result: out,
        diagnostics: diagnose_system(),
    }
}

fn main() {
    println!("ArchX v0.6 - Tauri Integration Example");
    println!("---------------------------------------");
    
    let a = vec![10.0; 100];
    let b = vec![20.0; 100];
    
    let response = archx_add(a, b);
    
    println!("Result length: {}", response.result.len());
    println!("Diagnostics:   {:?}", response.diagnostics);
    println!("---------------------------------------");
    println!("Integration successful.");
}
