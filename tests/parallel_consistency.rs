#[test]
fn test_parallel_consistency() {
    let size = 100_000;
    let a = vec![1.0; size];
    let b = vec![2.0; size];
    let mut out = vec![0.0; size];

    archx::add(&a, &b, &mut out);

    for (i, &val) in out.iter().enumerate().take(size) {
        assert_eq!(val, 3.0, "Mismatch at index {}", i);
    }
}
