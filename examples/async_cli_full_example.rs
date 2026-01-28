use archx::{add_async, WorkloadHints};

#[tokio::main]
async fn main() {
    println!("ArchX v0.8 - Async CLI Full Example");
    println!("--------------------------------");

    let size = 1_000_000;
    let a = vec![0.5; size];
    let b = vec![0.5; size];

    println!("Starting heavy async computation (1M elements)...");
    
    let handle = tokio::spawn(async move {
        add_async(a, b, WorkloadHints::default()).await
    });

    // Main thread is free to do other things
    println!("Main thread is performing other tasks while ArchX calculates...");
    
    let result = handle.await.unwrap();
    
    println!("Computation complete. Sum of first: {}", result[0]);
    println!("--------------------------------");
}
