# ArchX Sovereign — The Intelligence of Performance.

[![Crates.io](https://img.shields.io/crates/v/archx.svg)](https://crates.io/crates/archx)
[![Documentation](https://docs.rs/archx/badge.svg)](https://docs.rs/archx)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/Codevora-Studio/ArchX)
![v2.0 Sovereign](https://img.shields.io/badge/release-v2.0.0--sovereign-purple)

**ArchX v2.0 Sovereign** is a major evolution of the adaptive optimization engine. It provides a unified, cross-platform architecture for routing high-performance workloads across heterogeneous hardware (CPUs, GPUs, and SIMD silos).

---

## 💎 What's New in Sovereign (v2.0)

- **Unified SystemInfo**: A single source of truth for your entire hardware topology. ArchX now understands your CPU bitness, SIMD depth (SSE2 to AVX-512 & Neon), and connected GPU backends out of the box.
- **Optimized Instruction Paths**: Complete audit and rewrite of SIMD paths (AVX2, AVX-512, Neon) with loop unrolling and quad-vector loads for maximum throughput.
- **Heterogeneous Heuristics**: Our adaptive engine is now smarter. It balances workloads based on PCIe overhead, cache-line alignment, and P-core/E-core distributions.
- **Sovereign Profiling**: Multi-device, high-fidelity profiling with terminal-enhanced visual summaries and detailed CSV/JSON exports.

---

## 🚀 Sovereign Start

### Unified Detection
```rust
use archx::get_system_info;

fn main() {
    let info = get_system_info();
    println!("Platform: {:?} on {:?}", info.cpu.bits, info.cpu.arch);
    if let Some(gpu) = info.gpu {
        println!("GPU Acceleration Ready: {} ({}GB)", gpu.name, gpu.memory_gb);
    }
}
```

### The Fluent Engine (v2.0)
```rust
use archx::{engine, PowerMode};

fn main() {
    engine()
        .with_profile(true)          // Audit execution devices
        .with_limits(0.8)            // Utilize up to 80% cores
        .with_power_mode(PowerMode::HighPerformance)
        .add(&a, &b, &mut out);      // Intelligent auto-routing
}
```

---

## 🏗️ v2.0 Capability Matrix

| Feature | ArchX v1.x | ArchX v2.0 Sovereign |
| :--- | :--- | :--- |
| **SIMD** | x86 only | **x86 + AArch64 (Neon)** |
| **GPU Awareness** | Mock/Static | **Unified & Dynamic** |
| **Heuristics** | Size-based | **Topology-aware** |
| **Profiling** | CPU Only | **System-Wide (CPU/GPU)** |
| **Optimizations** | Basic SIMD | **Unrolled & Pipelined** |

---

## 🛡️ Trust & Performance

- **100% Rust-Safe Abstractions**: No manual `unsafe` required for users.
- **Zero-Dependency Core**: Only `std` and `serde`.
- **Deterministic Paths**: Predictable performance for real-time applications.

---

## 🤝 Contribution
Designed with love by **Codevora Studio**. ArchX is built by the community, for the community. See [CONTRIBUTING.md](./CONTRIBUTING.md) to join the Sovereignty.

---

## 🧪 Testing All Examples

To ensure your environment is fully compatible, you can run all included examples with a single command:

**Check Compilation:**
```bash
cargo check --examples
```

**Run All (PowerShell):**
```powershell
Get-ChildItem examples/*.rs | ForEach-Object { $name = $_.BaseName; Write-Host "Running: $name"; cargo run --example $name }
```

**Run All (Linux/macOS):**
```bash
for e in examples/*.rs; do name=$(basename $e .rs); echo "Running $name"; cargo run --example $name; done
```

---
MIT / Apache-2.0 © 2026 Codevora Studio
