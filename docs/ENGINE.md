# Sovereign v2.0 Adaptive Heuristics Engine

ArchX v2.0 Sovereign uses a state-of-the-art **Adaptive Engine** to make intelligent, device-aware execution decisions in real-time.

## The Strategy Matrix (v2.0)

The engine evaluates hardware across a unified topology:
1.  **Dataset Size**: Element count determines fixed overhead penalties (PCIe context switches, thread spawning).
2.  **Unified Hardware Capability**: SIMD level (SSE2, AVX2, AVX-512, Neon) and Dynamic GPU availability.
3.  **Instruction-Level Parallelism**: Optimized unrolled paths (v2.0) for maximum instruction density.
4.  **Operational Hints**: `WorkloadHints` such as `PowerMode` and `max_cpu_usage`.

### Heuristic Thresholds

| Dataset Size | Configuration | Chosen Path | Logic |
| :--- | :--- | :--- | :--- |
| < 1,024 | Any | `ScalarFallback` | Minimal overhead for tiny datasets. |
| 1k - 32k | Balanced | `SingleThreadSimd` | Optimal SIMD utilization vs setup costs. |
| 32k - 1M | High Perf | `ParallelSimd` | Multi-core scaling for moderate workloads. |
| > 1M | GPU (if avail) | `GpuOffload` | Massive throughput justifications for PCIe latency. |

## Heterogeneous Distribution

- **Unified Info**: v2.0 uses `SystemInfo` to understand the relationship between physical cores, logical processors, and GPU backends.
- **Topological Balancing**: Smarter thread distribution for hybrid CPU architectures (Intel 12th+ Gen).

## Power Management

- **Balanced**: Standard thresholds for general responsiveness.
- **HighPerformance**: Aggressively uses all logical cores and GPU units.
- **PowerSaving**: Higher thresholds for multithreading; favors wide SIMD on fewer cores to keep TDP/Battery usage low.
