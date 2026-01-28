use archx::*;
use std::time::{Instant, Duration};

fn run_benchmark(size: usize, name: &str) {
    let a = vec![1.1f32; size];
    let b = vec![2.2f32; size];
    let mut out = vec![0.0f32; size];
    
    println!("--- Benchmark: {} ({} elements) ---", name, size);
    
    // 1. Sync
    let start = Instant::now();
    for _ in 0..10 {
        archx::add(&a, &b, &mut out);
    }
    let duration = start.elapsed() / 10;
    let throughput = (size * 4 * 3) as f64 / 1e9 / duration.as_secs_f64();
    
    println!("Sync Throughput: {:.2} GB/s | Latency: {:?}", throughput, duration);
    
    // 2. Async (Simulated)
    // In v0.8 we use tokio for this
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let start_async = Instant::now();
    rt.block_on(async {
        add_async(a, b, WorkloadHints::default()).await
    });
    println!("Async Completion: {:?}", start_async.elapsed());
    println!("------------------------------------\n");
}

fn main() {
    println!("ArchX v0.8 Full Performance Suite");
    println!("==================================");
    
    run_benchmark(1_000_000, "Standard 1M");
    run_benchmark(10_000_000, "Large 10M");
    run_benchmark(100_000_000, "Massive 100M");
}
