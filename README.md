# ArchX Sovereign v2.1 — Hybrid Acceleration Engine
### Hybrid by Intelligence. Sovereign by Design.

[![Crates.io](https://img.shields.io/crates/v/archx.svg)](https://crates.io/crates/archx)
[![Documentation](https://docs.rs/archx/badge.svg)](https://docs.rs/archx)
![v2.1 Sovereign](https://img.shields.io/badge/release-v2.1--hybrid--sovereign-purple)
![Performance](https://img.shields.io/badge/speed-hybrid--optimized-orange)

**ArchX v2.1** is a high-performance acceleration framework that gives your Rust applications "Sovereignty of Execution." It doesn't just run code; it analyzes your hardware (CPU, iGPU, dGPU) and adaptively chooses the optimal parallel path.

---

## 🚀 What's New in v2.1 (Sovereign Hybrid)

### 1. Hybrid Device Discovery
The new `DeviceManager` distinguishes between **Integrated GPUs (iGPU)** like AMD Vega / Intel UHD and **Dedicated GPUs (dGPU)**. It maps shared memory topology to ensure zero-copy overhead where possible.

### 2. Cooperative Hybrid Dispatch
Large workloads (e.g., 10M+ elements) are now split between the GPU (70%) and CPU (30%) automatically. Result reconciliation is handled by the `HybridScheduler` with 100% safe fallback to SIMD if backends disappear.

### 3. Sovereign Policy Engine
Move beyond manual thread counts. Use high-level intent:
- `Policy::SmartAuto`: Dynamic CPU load & memory pressure analysis.
- `Policy::Performance`: Aggressive CPU+GPU cooperative scaling.
- `Policy::PowerSaving`: Efficient SIMD on P-cores with GPU idling.

### 4. Profiler v2.1 (Energy & Backend Tracking)
The upgraded profiler provides precise backend attribution (Vulkan/OpenCL/SIMD) and estimates energy consumption per task.

---

## 🏗️ Quick Start: Sovereign API

```rust
use archx::{archx, Policy};

fn main() {
    let a = vec![1.0; 10_000_000];
    let b = vec![2.0; 10_000_000];
    let mut out = vec![0.0; 10_000_000];

    archx()
        .policy(Policy::SmartAuto)
        .enable_gpu(true)
        .enable_hybrid(true)      // CPU + GPU Cooperative execution
        .profile(true)            // Detailed v2.1 analytics
        .execute(&a, &b, &mut out);
}
```

---

## 📊 v2.1 Capability Matrix

| Feature | ArchX v2.0 | ArchX v2.1 Sovereign Hybrid |
| :--- | :--- | :--- |
| **Execution** | Single Device | **CPU + GPU Cooperative** |
| **GPU Support** | dGPU / Generic | **iGPU (Vega/UHD) Optimized** |
| **Dispatch** | Strategy-based | **Policy-driven AI (SmartAuto)** |
| **Profiling** | Time-based | **Energy & Backend Attribution** |
| **WASM** | Static Fallback | **WASM SafeMode v2.1** |

---

## 🤝 The Sovereign Identity

ArchX v2.1 is designed for serious system-level acceleration. 
- **No Hard Dependencies**: Vulkan/OpenCL are dynamically loaded.
- **Panic-Free Policy**: If a driver fails, execution falls back in 1ms to CPU.
- **Enterprise Grade**: Ready for Tauri, WASM, and high-frequency CLI tools.

---

## 🧪 Bulk Verification

**Check Compilation:**
```bash
cargo check --examples
```

**Run All (PowerShell):**
```powershell
Get-ChildItem examples/*.rs | ForEach-Object { $name = $_.BaseName; cargo run --example $name }
```

---
Designed with love by **Codevora Studio**.
MIT / Apache-2.0 © 2026 Codevora Studio
