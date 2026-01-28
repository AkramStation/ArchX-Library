/// ArchX v2.1 Sovereign WASM Integration Example (Logic Only)
/// 
/// NOTE: To compile for WASM, use `wasm-pack build`.
/// This example shows the pattern for exposing ArchX to JS.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_add(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    let mut out = vec![0.0; a.len()];
    // Auto-dispatch handles detection within the WASM environment
    archx::add(&a, &b, &mut out);
    out
}

fn main() {
    println!("ArchX v2.1 - Sovereign WASM Example");
    println!("Target Architecture: {}", std::env::consts::ARCH);
    println!("To test WASM properly, follow the guide in README.md");
}
