# Profiling & Diagnostics Guide

ArchX Sovereign v3.0 provides deep visibility into the execution pipeline through the `profiling` module.

## Enabling Profiling

Profiling is disabled by default to ensure zero overhead in production. To enable it:

```rust
use archx::profiling::get_profiler;

### Pattern: Builder-Based Profiling (Sovereign v3.0)
The easiest way to profile a block is via the `ArchXBuilder`:

```rust
use archx::{ArchX, Policy};

ArchX::adaptive()
    .with_profile(true) // Activates profiler for this task
    .task(|| {
        // High-performance work
    })
    .execute();
```

## Interpreting Metrics

A `Metric` contains:
- **`name`**: The specific operation segment (e.g., "Parallel Chunk").
- **`subsystem`**: "Execution", "Memory", or "IO".
- **`duration`**: Precise `std::time::Duration`.
- **`thread_id`**: Which CPU core handled the task.
- **`gpu_device`**: (Optional) The name of the GPU backend used (Vulkan/OpenGL).
- **`vram_usage`**: (Optional) Estimated video memory consumed.

## Exporting Data

ArchX v3.0 utilizes the `ReportExporter` trait for flexible metric persistence.

### JSON (Full Diagnostic Data)
Ideal for deep analysis or integration with dashboards.
```rust
use archx::{get_profiler, JsonExporter, ReportExporter};

let data = get_profiler().get_metrics();
JsonExporter.export(&data, "archx_report.json")?;
```

### CSV (For Spreadsheet Analysis)
Optimized for performance review over many runs.
```rust
use archx::{get_profiler, CsvExporter, ReportExporter};

let data = get_profiler().get_metrics();
CsvExporter.export(&data, "archx_report.csv")?;
```

## Best Practices

1.  **Scope Profiling**: Clear the profiler between long-running ops to avoid memory accumulation.
2.  **Sample Rates**: In high-frequency loops, only enable profiling for every 100th iteration to minimize impact.
