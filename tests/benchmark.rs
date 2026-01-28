use archx;
use std::time::Instant;

#[test]
#[ignore]
fn benchmark_simd_performance() {
    let size = 10_000_000;
    let a = vec![1.23f32; size];
    let b = vec![4.56f32; size];
    let mut out = vec![0.0f32; size];

    let info = archx::get_info();
    println!("\n--- ArchX Performance Benchmark ---");
    println!("Size: {} floats", size);
    println!("Environment: {:?} (AVX2: {}, AVX: {}, SSE2: {})", 
        info.arch, info.features.avx2, info.features.avx, info.features.sse2);

    // Warm up
    archx::add(&a, &b, &mut out);

    let iterations = 50;
    let start = Instant::now();
    for _ in 0..iterations {
        archx::add(&a, &b, &mut out);
    }
    let duration = start.elapsed();
    let avg = duration / iterations as u32;

    println!("Total time ({} iterations): {:?}", iterations, duration);
    println!("Average time per iteration:   {:?}", avg);
    
    // Throughput
    let bytes_processed = size * 4 * 3; // a, b, out are all f32 (4 bytes)
    let gb_per_sec = (bytes_processed as f64 / 1_000_000_000.0) / avg.as_secs_f64();
    println!("Estimated Throughput:         {:.2} GB/s", gb_per_sec);
    println!("-----------------------------------\n");
}
