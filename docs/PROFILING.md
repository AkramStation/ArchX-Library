# Profiling & Diagnostics Guide

ArchX v1.1 provides deep visibility into the execution pipeline through the `profiling` module.

## Enabling Profiling

Profiling is disabled by default to ensure zero overhead in production. To enable it:

```rust
use archx::profiling::get_profiler;

fn main() {
    let p = get_profiler();
    p.set_enabled(true);
    
    // ... compute ...
    
    let snapshot = p.get_snapshot();
    println!("Recorded {} metrics", snapshot.len());
}
```

## Interpreting Metrics

A `Metric` contains:
- **`name`**: The specific operation segment (e.g., "Parallel Chunk").
- **`subsystem`**: "Execution", "Memory", or "IO".
- **`duration`**: Precise `std::time::Duration`.
- **`thread_id`**: Which core handled the task.

## Exporting Data

ArchX supports two built-in export formats:

### JSON (For UI Dashboards/Tauri)
```rust
let json_report = get_profiler().to_json();
```

### CSV (For External Analysis)
```rust
let csv_data = get_profiler().to_csv();
```

## Best Practices

1.  **Scope Profiling**: Clear the profiler between long-running ops to avoid memory accumulation.
2.  **Sample Rates**: In high-frequency loops, only enable profiling for every 100th iteration to minimize impact.
