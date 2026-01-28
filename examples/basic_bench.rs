use archx;
use std::time::Instant;

fn main() {
    let size = 5_000_000;
    let a = vec![2.5f32; size];
    let b = vec![3.5f32; size];
    let mut out = vec![0.0f32; size];

    println!("ArchX v0.3.1 - Basic Benchmark Example");
    println!("---------------------------------------");
    
    let info = archx::get_info();
    println!("Running on: {:?}", info.arch);
    println!("Features:   SSE2: {}, AVX: {}, AVX2: {}", 
        info.features.sse2, info.features.avx, info.features.avx2);

    // Initial call (warms up and initializes dispatch)
    let start_init = Instant::now();
    archx::add(&a, &b, &mut out);
    let init_duration = start_init.elapsed();
    println!("Initial call (detection + exec): {:?}", init_duration);

    // Subsequent calls (cached dispatch)
    let iterations = 100;
    let start_bench = Instant::now();
    for _ in 0..iterations {
        archx::add(&a, &b, &mut out);
    }
    let total_bench = start_bench.elapsed();
    let avg = total_bench / iterations;

    println!("Cached calls ({} iterations):  {:?}", iterations, total_bench);
    println!("Average per call:               {:?}", avg);
    println!("Throughput:                     {:.2} GB/s", 
        (size as f64 * 4.0 * 3.0 / 1_000_000_000.0) / avg.as_secs_f64());
    println!("---------------------------------------");
}
