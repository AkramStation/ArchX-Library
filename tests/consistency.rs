use archx;

// We need to access internal modules for granular parity testing.
// However, integration tests in 'tests/' can only see the public API.
// To test internal implementations against each other, we can:
// 1. Keep them in src/lib.rs (unit tests)
// 2. Or expose them as pub(crate) and use a 'test feature'
// For v0.3.1, we will re-implement the comparison logic using the public API 
// and internal knowledge if we want to be thorough, or keep the thorough 
// parity tests in lib.rs if they need private access.

#[test]
fn test_result_parity_various_sizes() {
    // Testing the public API 'add' which uses the best available path
    // against a simple manual loop (effectively the scalar logic).
    
    let sizes = [1, 3, 4, 7, 8, 15, 16, 31, 32, 1023, 1024, 1025];
    
    for &size in &sizes {
        let a: Vec<f32> = (0..size).map(|i| i as f32).collect();
        let b: Vec<f32> = (0..size).map(|i| (i * 2) as f32).collect();
        let mut out = vec![0.0; size];
        
        archx::add(&a, &b, &mut out);
        
        for i in 0..size {
            let expected = a[i] + b[i];
            // Use a small epsilon for float comparison if necessary, 
            // but for simple addition on integers cast to float, it should be exact.
            assert_eq!(out[i], expected, "Mismatch at index {} for size {}", i, size);
        }
    }
}
