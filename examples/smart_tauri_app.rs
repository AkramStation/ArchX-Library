use archx::*;

/// A Tauri-compatible command that uses the smart optimization engine.
/// 
/// WHY: v0.9 automatically balances CPU and GPU based on the environment,
/// making it ideal for cross-platform Tauri apps that may or may not have GPUs.
async fn tauri_smart_compute(data: Vec<f32>, power_saving: bool) -> Vec<f32> {
    let mut hints = WorkloadHints::default();
    if power_saving {
        hints.power_mode = PowerMode::PowerSaving;
    }
    
    // The engine automatically decides: SIMD-ST, SIMD-Parallel, or GPU
    add_async(data.clone(), data, hints).await
}

#[tokio::main]
async fn main() {
    println!("ArchX v0.9 - Smart Tauri-Ready Backend Example");
    println!("----------------------------------------------");
    
    let input = vec![1.23; 500_000];
    
    println!("Executing smart compute (Balanced)...");
    let res = tauri_smart_compute(input.clone(), false).await;
    println!("Compute 1 finished. Result[0]: {}", res[0]);

    println!("Executing smart compute (Power Saving)...");
    let res = tauri_smart_compute(input, true).await;
    println!("Compute 2 finished. Result[0]: {}", res[0]);
    
    println!("----------------------------------------------");
}
