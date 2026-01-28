/// Represents the word size of the CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Bitness {
    Bit32,
    Bit64,
}

/// Detects if the current target is 32-bit or 64-bit.
/// 
/// WHY: This is crucial for choosing between 32-bit and 64-bit optimized 
/// paths (e.g., using 64-bit registers for pointer manipulation).
pub fn detect_bits() -> Bitness {
    if cfg!(target_pointer_width = "64") {
        Bitness::Bit64
    } else {
        Bitness::Bit32
    }
}
