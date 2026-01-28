# ArchX — Your CPU already knows how to be fast. Let it decide.

[![Crates.io](https://img.shields.io/crates/v/archx.svg)](https://crates.io/crates/archx)
[![Documentation](https://docs.rs/archx/badge.svg)](https://docs.rs/archx)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/Codevora-Studio/ArchX)
![Production-Ready](https://img.shields.io/badge/status-production--ready-success)

**ArchX** is a mission-critical, adaptive acceleration engine for the modern Rust ecosystem. It eliminates the guesswork from performance optimization by dynamically routing workloads across SIMD, Multithreading, and GPU based on real-time hardware topology.

---

## ⚡ Why ArchX?

In the current landscape, software either settles for "good enough" performance or drowns in `unsafe` SIMD intrinsics and manual thread tuning. **ArchX changes the narrative.**

### The Problem
Traditional libraries (like Rayon) are excellent for data parallelism, but they don't care about your hardware's specific SIMD breadth (AVX2 vs AVX-512) or PCIe transfer overhead compared to CPU throughput. 

### The ArchX Solution
- **Adaptive Heuristics**: Decision-making based on L1/L2 cache sizes, memory bus throughput, and core topology.
- **Zero-Dependency Core**: Only `std` and `serde` (optional). No heavy runtime bloat.
- **Fluent Ecosystem**: A professional DX that lets you optimize in seconds, not hours.

---

## 🚀 Quick Start (30 Seconds)

### Basic Usage
The "Just Work" API for standard workloads.
```rust
use archx::add;

fn main() {
    let a = vec![1.0; 1_000_000];
    let b = vec![2.0; 1_000_000];
    let mut out = vec![0.0; 1_000_000];

    // ArchX detects your CPU and decides the best path (SIMD or Parallel).
    add(&a, &b, &mut out);
}
```

### Advanced Fluent API (v1.2)
Professional control for enterprise-grade applications.
```rust
use archx::engine;

fn main() {
    engine()
        .with_profile(true)        // Enable high-fidelity diagnostics
        .with_limits(0.4)          // Stay under 40% CPU usage
        .with_auto_gpu()           // Offload to GPU if PCIe overhead is justified
        .execute(|| {
            // Your heavy compute logic
        });
}
```

---

## 🏗️ Ecosystem Integration

| Integration | Guide |
| :--- | :--- |
| **🚀 Tauri** | [Building high-performance desktop apps with ArchX](./docs/INTEGRATION.md#tauri) |
| **🌐 WASM** | [SIMD-accelerated browser compute](./docs/INTEGRATION.md#wasm) |
| **⏳ Async** | [Non-blocking parallel processing for servers](./docs/INTEGRATION.md#async) |

---

## 📊 Comparison: ArchX vs The World

| Feature | Raw SIMD | Rayon | ArchX |
| :--- | :--- | :--- | :--- |
| **DX** | Hard (Unsafe) | Easy | **Fluent** |
| **Adaptive** | No | Partially | **Yes (Smart)** |
| **Hardware Aware** | Manual | No | **Yes (Automatic)** |
| **GPU Sync** | No | No | **Yes** |

---

## 🛡️ Trust & Reliability

- **Safe by Design**: 100% abstract over unsafe SIMD intrinsics.
- **Deterministic**: Heuristics are calibrated for stability, not just peak bursts.
- **Production Proven**: Benchmarked extensively on various hardware sets.

---

## 📅 v1.2 Roadmap
- [ ] **Plugin System**: custom optimizer plugins for specialized domains.
- [ ] **Dynamic Re-calibration**: live heuristic tuning based on system load.
- [ ] **ARM Neon parity**: full-speed optimizations for Apple Silicon.

## 🤝 Contribution
Designed with love by **Codevora Studio**. We welcome contributors! Check [CONTRIBUTING.md](./CONTRIBUTING.md) to get started.

---
MIT / Apache-2.0 © 2026 Codevora Studio
