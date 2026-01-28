use archx::profiling::get_profiler;
use std::time::Instant;

/// v2.1 Sovereign Benchmark & Profiling Suite
/// 
/// This tool allows users to verify ArchX performance on their own hardware
/// and see exactly how the adaptive engine behaves.
fn main() {
    println!("ArchX v2.1 - Professional Benchmark Suite");
    println!("========================================");
    
    let sizes = [10_000, 100_000, 1_000_000, 10_000_000];
    let profiler = get_profiler();
    profiler.set_enabled(true);

    for &size in &sizes {
        println!("\nTesting size: {} elements", size);
        let a = vec![1.0f32; size];
        let b = vec![2.0f32; size];
        let mut out = vec![0.0f32; size];
        
        profiler.clear();
        let start = Instant::now();
        archx::add(&a, &b, &mut out);
        let duration = start.elapsed();
        
        let throughput = (size * 4 * 3) as f64 / 1e9 / duration.as_secs_f64();
        println!("  Throughput: {:.2} GB/s", throughput);
        println!("  Analytics:  {} profiling metrics collected", profiler.get_snapshot().len());
    }

    println!("\nGenerating Performance Report (JSON)...");
    let report = profiler.to_json();
    println!("Report size: {} bytes", report.len());
    
    println!("========================================");
    println!("Verification Complete.");
}
