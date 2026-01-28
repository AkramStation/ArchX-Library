/// Represents specific CPU instruction set extensions.
#[derive(Debug, Clone, Copy, Default)]
pub struct CpuFeatures {
    /// Streaming SIMD Extensions 2
    pub sse2: bool,
    /// Advanced Vector Extensions
    pub avx: bool,
    /// Advanced Vector Extensions 2
    pub avx2: bool,
}

impl CpuFeatures {
    /// Detects available CPU features at runtime.
    /// 
    /// WHY: We use `is_x86_feature_detected!` which performs a runtime 
    /// check (e.g., via cpuid) to ensure the code doesn't crash on older CPUs.
    pub fn detect() -> Self {
        let mut features = CpuFeatures::default();

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            // SAFETY: These macros are safe to call at runtime as they handle 
            // OS-level support checks internally.
            features.sse2 = std::is_x86_feature_detected!("sse2");
            features.avx = std::is_x86_feature_detected!("avx");
            features.avx2 = std::is_x86_feature_detected!("avx2");
        }

        features
    }
}
