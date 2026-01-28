use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ArchXCoreError {
    /// Hardware detection or state capture failed.
    DetectionError(String),
    /// GPU backend failure (Allocation, Kernel compilation, Execution).
    GpuFailure(String),
    /// CPU SIMD dispatch or execution failure.
    SimdFailure(String),
    /// Scheduler was unable to dispatch task due to resource exhaustion or invalid strategy.
    SchedulerError(String),
    /// Resource management failure (e.g., OOM on GPU, thread limit reached).
    ResourceError(String),
    /// Arithmetic result is outside representable range.
    MathOverflow,
    /// Invalid input provided to the engine.
    InvalidInput(String),
    /// A general engine failure.
    EngineFault(String),
}

impl fmt::Display for ArchXCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArchXCoreError::DetectionError(msg) => write!(f, "Core Detection Error: {}", msg),
            ArchXCoreError::GpuFailure(msg) => write!(f, "Core GPU Failure: {}", msg),
            ArchXCoreError::SimdFailure(msg) => write!(f, "Core SIMD Failure: {}", msg),
            ArchXCoreError::SchedulerError(msg) => write!(f, "Core Scheduler Error: {}", msg),
            ArchXCoreError::ResourceError(msg) => write!(f, "Core Resource Error: {}", msg),
            ArchXCoreError::MathOverflow => write!(f, "Core Math Overflow"),
            ArchXCoreError::InvalidInput(msg) => write!(f, "Core Invalid Input: {}", msg),
            ArchXCoreError::EngineFault(msg) => write!(f, "Core Engine Fault: {}", msg),
        }
    }
}

impl std::error::Error for ArchXCoreError {}

pub type CoreResult<T> = Result<T, ArchXCoreError>;
