use archx::{archx, Policy, get_profiler};

fn main() {
    println!("--- ArchX v2.1 Sovereign Hybrid Demo ---");
    println!("Target: Hybrid Acceleration (CPU + iGPU)\n");

    let size = 10_000_000; // 10M elements to trigger Hybrid path
    let a = vec![1.0; size];
    let b = vec![2.0; size];
    let mut out = vec![0.0; size];

    println!("Executing with SmartAuto Policy...");
    
    // v2.1 Sovereign API
    archx()
        .policy(Policy::SmartAuto)
        .enable_gpu(true)
        .enable_hybrid(true)
        .profile(true)
        .execute(&a, &b, &mut out);

    println!("\nVerification: out[0] = {}, out[last] = {}", out[0], out[size-1]);
    
    // Print the v2.1 professional profile table
    get_profiler().print_summary();
}
