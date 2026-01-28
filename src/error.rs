use std::fmt;

/// The primary error type for all ArchX operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ArchXError {
    /// Failure during GPU resource allocation, kernel compilation, or execution.
    /// Includes descriptive message from the underlying GPU driver (CUDA/Vulkan).
    GpuError(String),
    /// Failure during SIMD dispatch or vector execution. 
    /// Typically occurs when hardware unsupported instructions are requested.
    SimdError(String),
    /// The provided input is malformed. 
    /// Common causes: Slice length mismatches (e.g., in `add(a, b, out)`).
    InvalidInput(String),
    /// Arithmetic result is outside the representable range (Overflow or Infinity).
    /// Only triggered when using `MathMode::Safe`.
    ArithmeticOverflow,
    /// Hardware detection or initialization failed.
    /// Occurs if system capability discovery cannot access required OS subsystems.
    HardwareError(String),
    /// A general execution failure within the task runner or hybrid scheduler.
    ExecutionError(String),
}

impl fmt::Display for ArchXError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArchXError::GpuError(msg) => write!(f, "GPU Error: {}", msg),
            ArchXError::SimdError(msg) => write!(f, "SIMD Error: {}", msg),
            ArchXError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
            ArchXError::ArithmeticOverflow => write!(f, "Arithmetic Overflow/Infinity encountered"),
            ArchXError::HardwareError(msg) => write!(f, "Hardware Error: {}", msg),
            ArchXError::ExecutionError(msg) => write!(f, "Execution Error: {}", msg),
        }
    }
}

impl std::error::Error for ArchXError {}

pub type ArchXResult<T> = Result<T, ArchXError>;
