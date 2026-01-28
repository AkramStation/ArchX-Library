# ArchX Sovereign v2.4 — Adaptive Performance Runtime
### Adaptive Intelligence. Sovereign Performance.

[![Crates.io](https://img.shields.io/crates/v/archx.svg)](https://crates.io/crates/archx)
[![Documentation](https://docs.rs/archx/badge.svg)](https://docs.rs/archx)
![v2.4 Sovereign](https://img.shields.io/badge/release-v2.4--adaptive--sovereign-blue)
![Performance](https://img.shields.io/badge/speed-simd--rayon--parallel-orange)

**ArchX v2.4** is the next evolution of Rust high-performance computing. It transforms from a simple acceleration engine into an **Adaptive Performance Runtime** that intelligently balances CPU instructions (SIMD), multi-core distribution (Work-Stealing), and device safety.

---

## 🚀 What's New in v2.4 (Sovereign Performance)

### 1. Dynamic SIMD Dispatch
ArchX now performs runtime feature detection to select the widest vector paths automatically.
- **x86/x64**: SSE2, AVX, AVX2, and AVX-512 support via gated intrinsics.
- **ARM**: Neon acceleration for mobile and server chips.
- **Zero Overhead**: Optimal function pointers are cached after the first detection.

### 2. Rayon Work-Stealing Scheduler
Integrated **Rayon** as the default parallel backbone.
- Replaces standard thread spawning with a lock-free work-stealing scheduler.
- Dramatically reduces overhead for fine-grained tasks.
- Improved cache locality and thread-safety.

### 3. Overflow-Safe Arithmetic Layer
High-frequency math operations are now shielded by the `SafeMath` trait.
- Detects integer overflows and float infinities at runtime.
- Use `ArchX::math()` to access safe, adaptive calculation paths.

### 4. Advanced Reporting & Exporters
Export performance data for analysis or CI/CD pipelines.
- **JSON Exporter**: Deep serialization of hardware state and task metrics.
- **CSV Exporter**: Legacy compatibility for spreadsheet-based profiling.

---

## 🏗️ Quick Start: New Task API

ArchX v2.4 introduces a closure-based task API that handles resource scaling for you:

```rust
use archx::{ArchX, Policy};

fn main() {
    // 1. Simple Managed Run
    ArchX::run(|| {
        println!("ArchX is managing this task based on system load...");
    });

    // 2. Fluent Builder for Fine Control
    ArchX::adaptive()
        .with_policy(Policy::Performance)
        .with_profile(true)
        .task(|| {
             // Your heavy computation here
             println!("Executing with Work-Stealing + SIMD Dispatch");
        })
        .execute();
}
```

---

## 📊 v2.4 Capability Matrix

| Feature | ArchX v2.3 | ArchX v2.4 Sovereign Performance |
| :--- | :--- | :--- |
| **Parallelism** | std::thread | **Rayon Work-Stealing** |
| **SIMD Dispatch** | Static/Manual | **Dynamic Runtime Detection** |
| **Arithmetic** | Native Rust | **Overflow-Safe (Trait Protected)** |
| **Diagnostics** | Console Summary | **JSON/CSV Serialized Reports** |
| **Policy Engine** | Hardware-Aware | **System-Load & Thermal Aware** |

---

## 🤝 The Sovereign Identity

ArchX v2.4 is built for stability and extreme performance.
- **Panic-Free**: All common math paths are protected against overflow.
- **Zero-Cost Abstractions**: The hardware dispatcher overhead is < 100ns per batch.
- **Backward Compatible**: Your v2.1-v2.3 `add` and `hybrid` APIs remain 100% functional.

---

## 🧪 Bulk Verification

**Check Compilation:**
```bash
cargo check --examples
```

**Run Flagship v2.4 Demo:**
```powershell
cargo run --example v2_4_sovereign_perf
```

---
Designed with love by **AkramStation**.
MIT / Apache-2.0 © 2026 AkramStation
