use archx::{ArchX, Policy};

fn main() {
    println!("--- ArchX v2.3 Adaptive Intelligence Demo ---");
    
    println!("\n1. Running a balanced task on an idle system...");
    ArchX::run(|| {
        let mut _sum: u64 = 0;
        for i in 0..1_000_000 { _sum += i as u64; }
        println!("Task 1 result computed.");
    });

    println!("\n2. Running with Performance Policy...");
    ArchX::adaptive()
        .with_policy(Policy::Performance)
        .task(|| {
            let mut _sum: u64 = 0;
            for i in 0..1_000_000 { _sum += i as u64; }
            println!("Task 2 (Performance) finished.");
        })
        .execute();

    println!("\n3. Running with ProtectDevice Policy (Simulating high pressure)...");
    ArchX::adaptive()
        .with_policy(Policy::ProtectDevice)
        .task(|| {
            println!("Task 3 (Protected) finished. ArchX should have throttled this.");
        })
        .execute();

    println!("\n4. Running with SmartAuto Policy...");
    ArchX::adaptive()
        .with_policy(Policy::SmartAuto)
        .task(|| {
            println!("Task 4 (SmartAuto) finished.");
        })
        .execute();
        
    println!("\nDemo complete. ArchX observed the system state and chose the safest path each time.");
}
