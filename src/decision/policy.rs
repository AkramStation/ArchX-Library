use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Policy {
    Performance,
    Balanced,
    PowerSaving,
    ProtectDevice, // Special mode for thermal/battery issues
    SmartAuto,     // Backward compatibility for v2.1
}

impl Default for Policy {
    fn default() -> Self {
        Policy::Balanced
    }
}
