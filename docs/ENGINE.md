# ArchX v3.0 Sovereign Fluent Engine

The ArchX v3.0 engine is a **Unified Hybrid Runtime**. It consolidates CPU SIMD, multi-core parallelism, and GPU Compute into a single, chainable fluent interface that prioritizes safety and developer ergonomics.

## Core Pillars (v3.0)

### 1. Unified Sovereign Builder
v3.0 replaces multiple disparate builders with the `SovereignBuilder`. Accessed via `ArchX::compute()`, it serves as the universal orchestration point for all tasks.

### 2. Error-First Design
Every mathematical and compute operation in v3.0 returns an `ArchXResult`. This ensures that issues like slice mismatches, GPU context losses, or arithmetic overflows are handled as data rather than crashes.

### 3. Cooperative Hybrid Scheduling
The engine manages the "Crossover Point":
- **Small Tasks**: Executed on CPU SIMD to avoid bus latency.
- **Large Tasks**: Split (70/30) between GPU and CPU to maximize total system throughput.
- **Policy Influence**:
    - `Policy::Performance`: Prioritizes GPU offloading for even moderately sized tasks.
    - `Policy::Balanced`: Uses a heuristics-based split to minimize total energy per computation.
    - `Policy::PowerSaving`: Restricts computation to the CPU to avoid data exposure on shared GPU buses.

## The v3.0 API Standard

| Feature | Legacy (v2.x) | Sovereign (v3.0) |
| :--- | :--- | :--- |
| **Entry Point** | `ArchX::new()` | `ArchX::compute()` |
| **Chaining** | Limited | **Full Fluent Chain** |
| **Error Handling** | Panics/Boilerplate | **ArchXResult (Type-Safe)** |
| **Functional** | Imperative | **Map/Reduce Friendly** |

---
*ArchX v3.0: Sovereign Performance, Unified.*
