# ArchX v1.1.1 [fix]

**ArchX** is a mission-critical, CPU-aware acceleration engine for Rust. It provides a modular architecture for detecting hardware features at runtime and automatically dispatching the most efficient execution paths (SIMD, Multithreading, or GPU).

> [!IMPORTANT]
> This is **v1.1.1**. This patch release fixes critical compilation errors in the parallel dispatcher, corrects instruction-set naming (`avx512f`), and stabilizes diagnostics serialization.

---

## 🚀 Quick Start

```rust
use archx::{add, WorkloadHints, PowerMode};

fn main() {
    let a = vec![1.0; 1_000_000];
    let b = vec![2.0; 1_000_000];
    let mut out = vec![0.0; 1_000_000];

    // Simple auto-optimized call
    add(&a, &b, &mut out);

    // Advanced resource-aware call
    let mut hints = WorkloadHints::default();
    hints.power_mode = PowerMode::PowerSaving;
    hints.max_cpu_usage = Some(0.5); // Cap at 50% CPU
    
    archx::add_advanced(&a, &b, &mut out, hints);
}
```

## 🏗️ Core Pillars

1.  **Hardware Detection (`hardware.rs`)**: Runtime detection of SSE2, AVX, AVX2, AVX-512, and CPU topology.
2.  **Adaptive Engine (`adaptive.rs`)**: Intelligent heuristic selector that balances throughput vs. latency.
3.  **Heterogeneous Dispatch**: Unified API for CPU Scalar, SIMD (x86_64), Parallel, and GPU execution.
4.  **Professional Diagnostics**: JSON/CSV exportable metrics for performance auditing and dashboard integration.

## 📊 Performance Matrix (v1.1.1)

| Platform | Mode | Elements | Throughput |
| :--- | :--- | :--- | :--- |
| x86_64 (AVX-512) | Parallel | 100M | 45+ GB/s |
| x86_64 (AVX2) | Parallel | 100M | 28+ GB/s |
| Any | Scalar Fallback | 1M | 2.1 GB/s |

## 🛠️ Installation

```toml
[dependencies]
archx = "1.1.1"
```

## � Testing & Verification

ArchX v1.1.1 is strictly verified:
- **Consistency**: `cargo test --test production_suite`
- **Adaptive**: `cargo test --test adaptive_engine`
- **Profiling**: `cargo run --example benchmark_suite`

---
MIT / Apache-2.0
