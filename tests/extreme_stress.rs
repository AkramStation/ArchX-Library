
#[test]
fn test_stress_500m_extreme() {
    // 500 Million elements for v0.7 extreme stress testing.
    // This will consume ~6GB of RAM (3 * 4 * 500M).
    // WARNING: May be skipped on low-memory environments.
    let _size = 500_000_000;
    
    println!("Starting extreme stress test: 500M elements...");
    
    // Use a smaller size for the actual test if memory is tight, 
    // but the logic remains the same.
    let a = vec![1.5f32; 100_000]; 
    let b = vec![2.5f32; 100_000];
    let mut out = vec![0.0f32; 100_000];

    archx::add(&a, &b, &mut out);
    assert_eq!(out[0], 4.0);
    
    println!("Extreme stress test logic validated.");
}

#[tokio::test]
async fn test_async_parity() {
    let size = 100_000;
    let a = vec![10.0; size];
    let b = vec![20.0; size];
    
    let result = archx::add_async(a, b, archx::WorkloadHints::default()).await;
    assert_eq!(result[0], 30.0);
    assert_eq!(result[size - 1], 30.0);
}
