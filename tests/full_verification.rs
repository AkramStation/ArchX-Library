use archx::*;

#[test]
fn test_universal_consistency() {
    let size = 100_000;
    let a = vec![2.0f32; size];
    let b = vec![3.0f32; size];
    let mut out_scalar = vec![0.0f32; size];
    let mut out_simd = vec![0.0f32; size];
    
    // 1. Reference: Scalar execution (simulated via small fragments)
    for i in 0..size {
        out_scalar[i] = a[i] + b[i];
    }

    // 2. Standard Public API (Auto-dispatch: SIMD + ST or MT)
    archx::add(&a, &b, &mut out_simd);
    
    // Check parity
    for i in 0..size {
        if out_simd[i] != out_scalar[i] {
            panic!("Consistency failed at index {}: {} != {}", i, out_simd[i], out_scalar[i]);
        }
    }

    println!("Universal consistency verified for size {}.", size);
}

#[tokio::test]
async fn test_async_consistency() {
    let size = 50_000;
    let a = vec![5.0; size];
    let b = vec![5.0; size];
    
    let result = archx::add_async(a, b, WorkloadHints::default()).await;
    
    assert_eq!(result[0], 10.0);
    assert_eq!(result[size-1], 10.0);
    println!("Async execution consistency verified.");
}

#[test]
fn test_edge_cases() {
    // Empty arrays
    let a: [f32; 0] = [];
    let mut out: [f32; 0] = [];
    archx::add(&a, &a, &mut out);
    
    // Small odd size
    let a = vec![1.0; 7];
    let mut out = vec![0.0; 7];
    archx::add(&a, &a, &mut out);
    assert_eq!(out[0], 2.0);
    assert_eq!(out[6], 2.0);
    
    println!("Edge cases verified.");
}
