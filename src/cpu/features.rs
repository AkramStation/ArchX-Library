/// Represents specific CPU instruction set extensions.
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct CpuFeatures {
    /// Streaming SIMD Extensions 2
    pub sse2: bool,
    /// Advanced Vector Extensions
    pub avx: bool,
    /// Advanced Vector Extensions 2
    pub avx2: bool,
    /// Advanced Vector Extensions 512 Foundation
    pub avx512f: bool,
    /// ARM NEON (Advanced SIMD)
    pub neon: bool,
}

impl CpuFeatures {
    /// Detects available CPU features at runtime.
    pub fn detect() -> Self {
        let mut features = CpuFeatures::default();

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            features.sse2 = std::is_x86_feature_detected!("sse2");
            features.avx = std::is_x86_feature_detected!("avx");
            features.avx2 = std::is_x86_feature_detected!("avx2");
            features.avx512f = std::is_x86_feature_detected!("avx512f");
        }

        #[cfg(target_arch = "aarch64")]
        {
            // Neon is mandatory on AArch64, but we can check specifically if needed
            // or just set to true on this platform.
            features.neon = true; 
        }

        features
    }
}
