use std::time::Instant;

#[test]
#[ignore]
fn benchmark_simd_performance() {
    let size = 10_000_000;
    let a = vec![1.23f32; size];
    let b = vec![4.56f32; size];
    let mut out = vec![0.0f32; size];

    let info = archx::get_info();
    println!("\n--- ArchX v0.6 Performance Benchmark ---");
    println!("Size: {} floats", size);
    println!("Environment: {:?} (AVX512: {}, AVX2: {}, AVX: {}, SSE2: {})", 
        info.arch, info.features.avx512f, info.features.avx2, info.features.avx, info.features.sse2);

    let iterations = 50;
    
    // Test Single-threaded (by forcing small size or just calling internal)
    // Actually, we can just run a large size and it will be MT.
    // To compare, we'd need to expose the internal ST dispatch or mock it.
    // For simplicity, we'll benchmark the public 'add' which is MT at this size.
    
    let start = Instant::now();
    for _ in 0..iterations {
        archx::add(&a, &b, &mut out);
    }
    let duration = start.elapsed();
    let avg = duration / iterations as u32;

    println!("Public API (Multi-threaded if size > 128k):");
    println!("Total time ({} iterations): {:?}", iterations, duration);
    println!("Average time per iteration:   {:?}", avg);
    
    // Throughput
    let bytes_processed = size * 4 * 3; 
    let gb_per_sec = (bytes_processed as f64 / 1_000_000_000.0) / avg.as_secs_f64();
    println!("Estimated Throughput:         {:.2} GB/s", gb_per_sec);
    
    // Note: To see the real difference, users can compare v0.3 to v0.4 
    // or we could add a flag to disable threading for benchmarking.
    println!("-----------------------------------\n");
}
