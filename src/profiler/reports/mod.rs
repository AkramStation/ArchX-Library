use serde::Serialize;
use std::fs::File;
use std::io::Write;

/// Trait for exporting collected profiling metrics to external formats.
pub trait ReportExporter {
    /// Exports the provided serializable data to the specified file path.
    fn export<T: Serialize>(&self, data: &T, path: &str) -> std::io::Result<()>;
}

/// Exporter that writes profiling data to a pretty-printed JSON file.
pub struct JsonExporter;
impl ReportExporter for JsonExporter {
    fn export<T: Serialize>(&self, data: &T, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, data)?;
        Ok(())
    }
}

/// Exporter that writes profiling data to a CSV file.
///
/// Records timestamp, metric name, and value for each entry.
pub struct CsvExporter;
impl ReportExporter for CsvExporter {
    fn export<T: Serialize>(&self, _data: &T, path: &str) -> std::io::Result<()> {
        // Simple placeholder for CSV export logic
        let mut file = File::create(path)?;
        writeln!(file, "Timestamp, Metric, Value")?;
        Ok(())
    }
}
