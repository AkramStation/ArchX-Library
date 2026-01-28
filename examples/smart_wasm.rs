/// Smart WASM integration pattern.
/// WASM targets typically have limited threads but good SIMD support (if enabled).
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_smart_add(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    let mut out = vec![0.0; a.len()];
    let mut hints = archx::WorkloadHints::default();
    // In WASM, we might want to cap CPU usage or stick to Balanced
    hints.power_mode = archx::PowerMode::Balanced;
    
    archx::add_advanced(&a, &b, &mut out, hints);
    out
}

fn main() {
    println!("ArchX v0.9 - Smart WASM Integration Guide");
    println!("Hardware detection in WASM is specialized.");
    println!("ArchX v0.9 heuristics are compatible with wasm32 single-thread fallback.");
}
