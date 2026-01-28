# Integration Guide: Tauri & WASM

ArchX Sovereign v3.0 is designed to be a "backend engine" for cross-platform applications.

## Tauri Integration

Tauri apps benefit from ArchX's async-by-default compute paths.

### Pattern: High-Performance Math (v3.0 Sovereign)
ArchX v3.0 provides a unified, chainable math API with built-in error handling:

```rust
#[tauri::command]
fn dot_product(a: Vec<f32>, b: Vec<f32>) -> Result<f32, String> {
    archx::ArchX::compute()
        .with_mode(archx::MathMode::Fast)
        .dot(&a, &b)
        .map_err(|e| e.to_string())
}
```

### Pattern: Hybrid GPU Computation (v3.0)
Leverage GPU compute with simple chaining and resilience:

```rust
#[tauri::command]
fn compute_hybrid(a: Vec<f32>, b: Vec<f32>) -> Result<Vec<f32>, String> {
    let mut out = vec![0.0; a.len()];
    archx::ArchX::compute()
        .with_gpu(archx::GpuPolicy::Adaptive)
        .add(&a, &b, &mut out)
        .map_err(|e| e.to_string())?;
    Ok(out)
}
```

### Pattern: Real-time Diagnostics
Export profiling JSON directly to the frontend for performance graphing:

```rust
#[tauri::command]
fn get_metrics() -> String {
    archx::profiling::get_profiler().to_json()
}
```

## WASM Integration

ArchX is compatible with `wasm32-unknown-unknown` targets.

### Limitations
- **Threading**: Standard Rust `std::thread` is unavailable in many WASM environments. ArchX automatically falls back to single-threaded SIMD or Scalar.
- **SIMD**: Requires the `simd128` feature to be enabled in your target RUSTFLAGS.

### Usage
```rust
#[wasm_bindgen]
pub fn fast_add(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    let mut out = vec![0.0; a.len()];
    archx::add(&a, &b, &mut out);
    out
}
```
