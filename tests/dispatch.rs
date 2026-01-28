use archx;

#[test]
fn test_dispatch_logic() {
    let info = archx::get_info();
    
    // This integration test ensures the library is in a runnable state
    // and correctly chooses an execution path.
    
    let a = vec![1.0; 100];
    let b = vec![2.0; 100];
    let mut out = vec![0.0; 100];
    
    archx::add(&a, &b, &mut out);
    
    for val in out {
        assert_eq!(val, 3.0);
    }
    
    println!("Dispatch test passed on {:?}", info.arch);
}
