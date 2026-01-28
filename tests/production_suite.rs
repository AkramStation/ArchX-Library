use archx::*;

#[test]
fn test_v1_0_engine_stability() {
    let a = vec![1.0; 1000];
    let mut out = vec![0.0; 1000];
    
    // Test that default add still works (backward compatibility)
    archx::add(&a, &a, &mut out);
    assert_eq!(out[0], 2.0);
    
    // Test that production power modes don't crash
    let hints = WorkloadHints {
        power_mode: PowerMode::PowerSaving,
        ..Default::default()
    };
    archx::add_advanced(&a, &a, &mut out, hints);
    assert_eq!(out[999], 2.0);
}

#[tokio::test]
async fn test_v1_0_async_parity() {
    let a = vec![5.0; 10_000];
    let b = vec![5.0; 10_000];
    let res = archx::add_async(a, b, WorkloadHints::default()).await;
    assert_eq!(res[0], 10.0);
}
