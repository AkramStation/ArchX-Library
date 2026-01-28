# ArchX Sovereign v3.0 — Fluent Performance Runtime
### Adaptive Intelligence. Sovereign Ergonomics.

[![Crates.io](https://img.shields.io/crates/v/archx.svg)](https://crates.io/crates/archx)
[![Documentation](https://docs.rs/archx/badge.svg)](https://docs.rs/archx)
![v3.0 Sovereign](https://img.shields.io/badge/release-v3.0--sovereign--fluent-blue)
![Performance](https://img.shields.io/badge/speed-simd--gpu--hybrid-orange)

**ArchX v3.0** is a high-performance, adaptive runtime library designed for systems programming in Rust. It introduces the **Sovereign Fluent API**, a unified, chainable interface for CPU, GPU, and Hybrid compute.

---

## 🚀 Quick Start

Add `ArchX` to your `Cargo.toml`:
```toml
[dependencies]
archx = "2.4"
```

### Basic Vector Addition
```rust
use archx::{ArchX, Policy};

fn main() -> Result<(), archx::ArchXResult<()>> {
    let a = vec![1.0; 1000];
    let b = vec![2.0; 1000];
    let mut out = vec![0.0; 1000];

    ArchX::compute()
        .with_policy(Policy::Balanced)
        .add(&a, &b, &mut out)?;
        
    Ok(())
}
```

---

## 🛠️ Core API & Execution Modes

ArchX supports multiple execution modes through a single entry point:

1. **CPU/SIMD**: Parallel execution using host threads and SIMD instructions (AVX2, AVX-512).
2. **GPU (Adaptive)**: Offloads heavy computation to GPU backends (Vulkan/CUDA).
3. **Hybrid**: Cooperative scheduling that splits workloads between CPU and GPU.
4. **Async**: Non-blocking background execution.

### GPU Configuration
```rust
use archx::{archx, GpuPolicy};

archx()
    .with_gpu(GpuPolicy::ForceGpu) // Force GPU execution
    .sum(&data)?;
```

---

## 📊 Hybrid Execution & Profiling

ArchX intelligently splits workloads based on hardware state and workload size.

### Collecting Metrics
```rust
use archx::{archx, JsonExporter, ReportExporter};

let metrics = archx()
    .profile(true)
    .with_policy(Policy::Performance)
    .run(|| {
        // Your complex task
    });

// Export metrics to JSON
let exporter = JsonExporter;
exporter.export(&metrics, "performance_report.json")?;
```

---

## 🧮 Mathematical Operations

The Fluent API provides high-level math primitives:

| Method | Operation | Description |
|--------|-----------|-------------|
| `add` | `out = a + b` | Vectorized element-wise addition |
| `sub` | `out = a - b` | Vectorized element-wise subtraction |
| `mul` | `out = a * b` | Vectorized element-wise multiplication |
| `dot` | `sum(a * b)` | Scalar dot product |
| `sum` | `sum(a)` | Parallel reduction sum |

### Safety Modes
```rust
use archx::{archx, MathMode};

archx()
    .with_mode(MathMode::Safe) // Enable overflow checking
    .mul(&a, &b, &mut out)?;
```

---

## ⚡ Advanced Usage

### Asynchronous Operations
Avoid blocking the main thread for massive datasets:
```rust
use archx::{add_async, WorkloadHints};

#[tokio::main]
async fn main() {
    let hints = WorkloadHints { prefer_gpu: true, ..Default::default() };
    let result = add_async(vec![1.0; 10_000], vec![2.0; 10_000], hints).await;
}
```

---

## ⚠️ Error Handling

All ArchX operations return `ArchXResult<T>`. Handle errors gracefully:

```rust
match archx().add(&a, &b, &mut out) {
    Ok(_) => println!("Success!"),
    Err(e) => match e {
        ArchXError::InvalidInput(msg) => eprintln!("Input error: {}", msg),
        ArchXError::GpuError(msg) => eprintln!("GPU failure: {}", msg),
        ArchXError::ArithmeticOverflow => eprintln!("Result too large!"),
        _ => eprintln!("ArchX Error: {:?}", e),
    }
}
```

---

## 🧪 Verification
Run flagship demo:
```bash
cargo run --example v3_fluent_api_demo
```

Designed with ❤️ by **AkramStation**.
MIT / Apache-2.0 © 2026 AkramStation
