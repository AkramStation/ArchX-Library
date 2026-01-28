use serde::Serialize;
use std::fs::File;
use std::io::Write;

pub trait ReportExporter {
    fn export<T: Serialize>(&self, data: &T, path: &str) -> std::io::Result<()>;
}

pub struct JsonExporter;
impl ReportExporter for JsonExporter {
    fn export<T: Serialize>(&self, data: &T, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, data)?;
        Ok(())
    }
}

pub struct CsvExporter;
impl ReportExporter for CsvExporter {
    fn export<T: Serialize>(&self, _data: &T, path: &str) -> std::io::Result<()> {
        // Simple placeholder for CSV export logic
        let mut file = File::create(path)?;
        writeln!(file, "Timestamp, Metric, Value")?;
        Ok(())
    }
}
