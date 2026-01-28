use archx::{SafeMath, JsonExporter, ReportExporter, ArchXSched};
use serde::Serialize;

#[derive(Serialize)]
struct PerformanceReport {
    task: String,
    simd_active: bool,
    threads: usize,
    duration_ms: u128,
}

fn main() -> std::io::Result<()> {
    println!("--- ArchX v2.4 Sovereign Performance Demo ---");

    let size = 1_000_000;
    let a = vec![1.0f32; size];
    let b = vec![2.0f32; size];
    let mut out = vec![0.0f32; size];

    println!("\n1. Parallel SIMD Addition (Rayon + SSE/AVX)...");
    let start = std::time::Instant::now();
    ArchXSched::parallel_add(&a, &b, &mut out);
    let duration = start.elapsed().as_millis();
    println!("Executed 1M elements in {}ms", duration);

    println!("\n2. Safe Arithmetic with Overflow Detection...");
    let result = f32::safe_add(3.4028235e38, 1.0e38);
    println!("Safe Add Result: value={}, overflowed={}", result.value, result.overflowed);

    println!("\n3. Generating Unified Performance Report (JSON)...");
    let report = PerformanceReport {
        task: "Parallel SIMD Add".to_string(),
        simd_active: true,
        threads: rayon::current_num_threads(),
        duration_ms: duration,
    };
    
    let exporter = JsonExporter;
    exporter.export(&report, "archx_report_v2.4.json")?;
    println!("Report saved to archx_report_v2.4.json");

    println!("\nv2.4: Intelligence that Scales with Power.");
    Ok(())
}
