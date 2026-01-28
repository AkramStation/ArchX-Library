use archx;

#[test]
fn test_stress_massive_workload() {
    // 100 Million elements for v0.5 stress testing.
    // This will consume ~1.2GB of RAM (3 * 4 * 100M).
    let size = 100_000_000;
    
    // We'll use a smaller size if RAM is an issue in some environments, 
    // but the request specifically asked for 100M+.
    let a = vec![1.25f32; size];
    let b = vec![2.75f32; size];
    let mut out = vec![0.0f32; size];

    println!("Starting stress test: 100M elements...");
    archx::add(&a, &b, &mut out);

    // Verify a few samples to ensure correctness across the entire range.
    assert_eq!(out[0], 4.0);
    assert_eq!(out[size / 2], 4.0);
    assert_eq!(out[size - 1], 4.0);
    println!("Stress test passed.");
}

#[test]
fn test_advanced_hints() {
    let size = 1000;
    let a = vec![1.0; size];
    let b = vec![2.0; size];
    let mut out = vec![0.0; size];

    let mut hints = archx::WorkloadHints::default();
    hints.thread_count = Some(2); // Force threading even for small size.
    
    archx::add_advanced(&a, &b, &mut out, hints);
    assert_eq!(out[0], 3.0);
}
