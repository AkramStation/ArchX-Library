use crate::math::MathMode;
use crate::gpu::GpuPolicy;

pub struct MathBuilder {
    mode: MathMode,
    profiling: bool,
    gpu_enabled: bool,
    gpu_policy: GpuPolicy,
}

impl MathBuilder {
    pub fn new() -> Self {
        Self {
            mode: MathMode::Balanced,
            profiling: false,
            gpu_enabled: false,
            gpu_policy: GpuPolicy::Adaptive,
        }
    }

    pub fn mode(mut self, mode: MathMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn profile(mut self, enabled: bool) -> Self {
        self.profiling = enabled;
        self
    }

    pub fn enable_gpu(mut self, enabled: bool) -> Self {
        self.gpu_enabled = enabled;
        self
    }

    pub fn with_gpu_policy(mut self, policy: GpuPolicy) -> Self {
        self.gpu_policy = policy;
        crate::gpu::set_gpu_policy(policy);
        self
    }

    /// Performs parallel addition.
    pub fn add(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        let _ = self.to_sovereign().add(a, b, out);
    }

    /// Performs parallel subtraction.
    pub fn sub(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        let _ = self.to_sovereign().sub(a, b, out);
    }

    /// Performs parallel multiplication.
    pub fn mul(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        let _ = self.to_sovereign().mul(a, b, out);
    }

    /// Performs parallel dot product.
    pub fn dot(self, a: &[f32], b: &[f32]) -> f32 {
        self.to_sovereign().dot(a, b).unwrap_or(0.0)
    }

    /// Performs parallel summation.
    pub fn sum(self, a: &[f32]) -> f32 {
        self.to_sovereign().sum(a).unwrap_or(0.0)
    }

    fn to_sovereign(self) -> crate::public_api::sovereign::SovereignBuilder {
        crate::public_api::ArchX::compute()
            .with_mode(self.mode)
            .enable_gpu(self.gpu_enabled)
            .with_gpu(self.gpu_policy)
            .profile(self.profiling)
    }
}
