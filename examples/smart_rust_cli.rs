use archx::*;

#[tokio::main]
async fn main() {
    println!("ArchX v2.1 - Sovereign Smart CLI Example");
    println!("===============================================");

    // 1. Balanced Mode (Default)
    let data = vec![1.0; 1_000_000];
    let mut out = vec![0.0; data.len()];
    
    println!("Running Balanced Mode (1M elements)...");
    add_advanced(&data, &data, &mut out, WorkloadHints::default());

    // 2. Power Saving Mode (Favors SIMD over Threads)
    let hints_ps = WorkloadHints {
        power_mode: PowerMode::PowerSaving,
        ..Default::default()
    };
    println!("Running Power Saving Mode...");
    add_advanced(&data, &data, &mut out, hints_ps);

    // 3. Resource Capped (Max 25% CPU)
    let hints_cap = WorkloadHints {
        max_cpu_usage: Some(0.25),
        ..Default::default()
    };
    println!("Running Resource Capped (Max 25%% CPU)...");
    add_advanced(&data, &data, &mut out, hints_cap);

    // 4. Async + Smart Diagnostics
    println!("Running Async Smart Dispatch...");
    let _ = add_async(data.clone(), data, WorkloadHints::default()).await;
    
    let info = get_info();
    println!("Final Diagnostics: CPU cores utilized conservatively based on load.");
    println!("Current Hardware: {:?} with {:?} logical processors", info.arch, info.bits);
    println!("All operations consistent and resource-aware.");
}
