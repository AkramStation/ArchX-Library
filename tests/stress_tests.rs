use archx;

#[test]
fn test_massive_scale() {
    // We'll try 100M first to ensure stability without swapping on common systems.
    // 1B elements would require ~12GB RAM for 3 f32 vectors.
    let size = 100_000_000;
    
    println!("Allocating massive vectors (100M f32 elements)...");
    let a = vec![1.0f32; size];
    let b = vec![2.0f32; size];
    let mut out = vec![0.0f32; size];
    
    println!("Executing parallel SIMD add...");
    let start = std::time::Instant::now();
    archx::add(&a, &b, &mut out);
    let duration = start.elapsed();
    
    println!("Massive execution finished in {:?}", duration);
    assert_eq!(out[0], 3.0);
    assert_eq!(out[size - 1], 3.0);
}

#[test]
fn test_misaligned_access() {
    // Note: Rust's Vec is usually 8 or 16 byte aligned.
    // We simulate misalignment by slicing.
    let full = vec![1.0f32; 100];
    let a = &full[1..33]; // Not 64-byte aligned
    let b = &full[5..37];
    let mut out = vec![0.0f32; 32];
    
    archx::add(a, b, &mut out);
    assert_eq!(out[0], 2.0);
    println!("Misaligned slicing verified.");
}
