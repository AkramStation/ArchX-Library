# Hardware Detection & SIMD Guide

ArchX v1.1 relies on deep hardware detection to choose the correct instruction sets at runtime.

## Core Hierarchy

ArchX organizes its backends in a priority-based hierarchy:

1.  **AVX-512**: 512-bit registers (16 floats per cycle). Highest throughput on modern Intel/AMD.
2.  **AVX2**: 256-bit registers with FMA support. The industry standard for performance.
3.  **AVX**: 256-bit registers (8 floats per cycle).
4.  **SSE2**: 128-bit registers (4 floats per cycle). Universal baseline for x86_64.
5.  **Scalar**: Single-instruction, single-data fallback.

## Detective Logic (`hardware.rs`)

The library uses the `cpuid` instruction (internally via the `raw_cpuid` crate logic) to check bit-flags for specific instruction sets. 

### Detection Order
Detection happens once during the first library call and is cached globally for subsequent operations.

## Topology Awareness

v1.1 introduces better logical processor vs. physical core detection:
- **Logical Processors**: Total available threads (including Hyperthreading).
- **Physical Cores**: Used as the primary target for `PowerSaving` mode to minimize thermal context-switching.
