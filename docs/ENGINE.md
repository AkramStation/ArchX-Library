# ArchX v2.4 Adaptive Performance Engine

The ArchX v2.4 engine has evolved from a simple heuristic selector into a **Real-Time Adaptive Runtime**. It doesn't just check hardware features; it monitors system load and thermal status to ensure performance never compromises stability.

## Core Optimization Pillars (v2.4)

### 1. Unified Hardware Topology
The engine maintains a deep map of the host machine:
- **Instruction Sets**: Dynamic SSE2, AVX, AVX2, AVX-512, and Neon detection.
- **Micro-Architecture**: Distinguishes between Performance (P) and Efficiency (E) cores (leveraging `rayon` core affinity).
- **GPU Backends**: Dynamic Vulkan/OpenCL loading with unified memory awareness.

### 2. Work-Stealing Scheduling
By switching to **Rayon**, v2.4 eliminates the "tail latency" problem found in standard thread spawning.
- **Theft-Based Balancing**: If one core is busier (due to background OS processes), other cores "steal" its tasks to finish work faster.
- **Cache Locality**: Rayon ensures that chunks are split logically to fit into L1/L2 caches.

### 3. Load-Aware Heuristics
The decision matrix now incorporates **System Load %** from the `detect` module.

#### v2.4 Execution Strategy Matrix

| Load % | Dataset Size | Policy | Final Strategy |
| :--- | :--- | :--- | :--- |
| Any | < 4,096 | Any | **Scalar/SIMD ST** (Avoid thread overhead) |
| < 40% | > 32k | Performance | **Rayon Parallel (Full Capacity)** |
| 40-80% | > 32k | Balanced | **Rayon Parallel (50% Threads)** |
| > 90% | Any | Any | **Throttled (Safe Sequential)** |
| Any | > 5M | SmartAuto | **iGPU/Hybrid Dispatch** |

## Intelligent SIMD Dispatching
v2.4 uses a **Function Pointer Cache (FPC)**. Upon first execution:
1. `SimdDispatcher` queries `CpuFeatures`.
2. It selects the widest available SIMD width (e.g., AVX2).
3. It locks that function pointer in memory.
4. Subsequent calls are direct function jumps with zero branching overhead.

## Device Protection Governance
- **Thermal Awareness**: When `Policy::ProtectDevice` is active, the engine monitors `sysinfo` thermal thresholds.
- **Battery Scaling**: Automatically disables high-power GPU paths when on battery power (Windows/Linux only).

---
*ArchX v2.4: Sovereign Performance for the Modern Hardware Ecosystem.*
