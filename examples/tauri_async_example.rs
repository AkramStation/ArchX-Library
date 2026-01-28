use archx::{add_async, WorkloadHints};

// Imagine a Tauri #[tauri::command]
async fn tauri_archx_compute(data: Vec<f32>) -> Vec<f32> {
    let hints = WorkloadHints {
        prefer_gpu: true,
        ..Default::default()
    };
    
    // Non-blocking computation! Safe for UI threads.
    add_async(data.clone(), data, hints).await
}

#[tokio::main]
async fn main() {
    println!("ArchX v2.1 - Sovereign Tauri Async Integration Example");
    println!("---------------------------------------------");
    
    let large_data = vec![1.0; 1000];
    
    // Call the simulated Tauri command
    let result = tauri_archx_compute(large_data).await;
    
    println!("Compute task finished. Result size: {}", result.len());
    println!("Example value at [0]: {}", result[0]);
    println!("---------------------------------------------");
}
