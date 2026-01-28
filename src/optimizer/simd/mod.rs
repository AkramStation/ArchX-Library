//! SIMD (Single Instruction, Multiple Data) optimizations for ArchX.
//! 
//! This module contains architecture-specific optimizations.
//! 
//! ### v0.2 Implementation: SSE2
//! We start with SSE2 as it is the most widely supported SIMD extension 
//! on x86_64, ensuring a baseline performance boost for almost all modern PCs.
//! 
//! ### Future Expansion
//! - **AVX/AVX2**: Planned for v0.3 to double/quadruple throughput on supported CPUs.
//! - **AArch64 NEON**: Planned for future mobile/Apple Silicon support.

pub mod avx;
pub mod avx2;
pub mod sse2;
