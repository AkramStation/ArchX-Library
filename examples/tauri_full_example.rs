use archx::*;
use archx::profiling::{get_profiler, Metric};

/// Simulated Tauri Command that returns diagnostics and profiling data.
/// 
/// WHY: v0.8 provides transparency into how the library optimizes paths.
async fn archx_process_full(data: Vec<f32>) -> (Vec<f32>, Vec<Metric>) {
    let profiler = get_profiler();
    profiler.set_enabled(true);
    profiler.clear();
    
    let a = data.clone();
    let b = data.clone();
    let result = add_async(a, b, WorkloadHints::default()).await;
    
    let stats = profiler.get_snapshot();
    (result, stats)
}

#[tokio::main]
async fn main() {
    println!("ArchX v2.1 - Sovereign Tauri Full Integration & Profiling Example");
    println!("-------------------------------------------------------");
    
    let input = vec![1.23; 10_000_000];
    println!("Processing 10M elements with profiling enabled...");
    
    let (res, metrics) = archx_process_full(input).await;
    
    println!("Result computed. Array size: {}", res.len());
    println!("Profiling metrics collected: {}", metrics.len());
    
    for metric in metrics.iter().take(5) {
        println!(" - Operation: {:<15} | Duration: {:?}", metric.name, metric.duration);
    }
    
    if metrics.len() > 5 {
        println!("   ... and {} more metrics.", metrics.len() - 5);
    }
    
    println!("-------------------------------------------------------");
}
