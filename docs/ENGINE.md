# Adaptive Heuristics Engine

ArchX v1.1 uses a sophisticated **Adaptive Engine** to make execution decisions in real-time. This guide explains the logic used to select the most efficient compute path.

## The Strategy Matrix

The engine evaluates three primary vectors:
1.  **Dataset Size**: Element count determines fixed overhead penalties (PCIe context switches, thread spawning).
2.  **Hardware Capability**: SIMD level (SSE2 vs AVX-512) and GPU Availability.
3.  **User Constraints**: `WorkloadHints` such as `PowerMode` and `max_cpu_usage`.

### Heuristic Thresholds (v1.1)

| Dataset Size | Hardware | Chosen Path | Reason |
| :--- | :--- | :--- | :--- |
| < 1,000 | Any | `ScalarFallback` | Zero overhead is better than tiny SIMD gains. |
| 1k - 32k | SIMD | `SingleThreadSimd` | Thread spawning overhead exceeds compute gain. |
| 32k - 250k | Multi-core | `ParallelSimd` | Throughput scaling overcomes thread sync costs. |
| > 250k | GPU (if pref) | `GpuOffload` | Compute density justifies PCIe transfer latency. |

## Power Modes

- **Balanced**: Standard thresholds for general responsiveness.
- **HighPerformance**: Aggressively uses all logical cores and GPU units.
- **PowerSaving**: Higher thresholds for multithreading; favors wide SIMD on fewer cores to keep TDP low.

## Resource Capping

By setting `max_cpu_usage`, the engine dynamically limits the number of spawned worker threads in `parallel.rs`, ensuring ArchX stays within its assigned budget.
