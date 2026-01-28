# Integration Guide: Tauri & WASM

ArchX v1.1 is designed to be a "backend engine" for cross-platform applications.

## Tauri Integration

Tauri apps benefit from ArchX's async-by-default compute paths.

### Pattern: The Async Command
Avoid blocking the webview thread by using the ArchX async bridge:

```rust
#[tauri::command]
async fn compute_heavy(data: Vec<f32>) -> Result<Vec<f32>, String> {
    Ok(archx::add_async(data.clone(), data, WorkloadHints::default()).await)
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
