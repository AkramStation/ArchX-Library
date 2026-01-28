use archx;

#[test]
fn test_parallel_consistency() {
    // Force parallel processing by using a size larger than PARALLEL_THRESHOLD (128,000)
    let size = 200_000;
    let a: Vec<f32> = (0..size).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..size).map(|i| (i * 2) as f32).collect();
    let mut out = vec![0.0; size];
    
    archx::add(&a, &b, &mut out);
    
    for i in 0..size {
        let expected = i as f32 + (i * 2) as f32;
        if out[i] != expected {
            panic!("Mismatch at index {}: {} != {}", i, out[i], expected);
        }
    }
    
    // Test with non-aligned remainder to ensure no edge-case crashes
    let odd_size = 200_007;
    let a_odd = vec![1.0; odd_size];
    let b_odd = vec![2.0; odd_size];
    let mut out_odd = vec![0.0; odd_size];
    
    archx::add(&a_odd, &b_odd, &mut out_odd);
    assert_eq!(out_odd[odd_size - 1], 3.0);
}
