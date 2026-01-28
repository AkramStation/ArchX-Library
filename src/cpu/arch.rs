use std::env;

/// Represents supported CPU architectures.
/// Using an enum provides type safety across the library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArch {
    X86,
    X86_64,
    ARM,
    AArch64,
    Unknown,
}

/// Detects the CPU architecture at compile-time/runtime using standard constants.
/// 
/// WHY: We use `std::env::consts::ARCH` as it is the most reliable way 
/// to get the target architecture without external dependencies in v0.1.
pub fn detect_arch() -> CpuArch {
    match env::consts::ARCH {
        "x86" => CpuArch::X86,
        "x86_64" => CpuArch::X86_64,
        "arm" => CpuArch::ARM,
        "aarch64" => CpuArch::AArch64,
        _ => CpuArch::Unknown,
    }
}
