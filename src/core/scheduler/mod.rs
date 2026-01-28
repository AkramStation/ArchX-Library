use crate::core::error::{CoreResult, ArchXCoreError};
use crate::core::detect::HardwareState;
use crate::core::resource::ResourceManager;
use crate::gpu::GpuPolicy;
use std::sync::Arc;

pub trait UnifiedScheduler: Send + Sync {
    fn prepare_task(&self) -> CoreResult<()>;
    fn complete_task(&self);
    
    fn set_policy(&self, policy: crate::decision::Policy);
    fn set_gpu_policy(&self, policy: GpuPolicy);
}

impl dyn UnifiedScheduler {
    pub fn run<F, R>(&self, task: F) -> CoreResult<R>
    where F: FnOnce() -> R 
    {
        self.prepare_task()?;
        let result = task();
        self.complete_task();
        Ok(result)
    }
}

pub struct DefaultScheduler {
    resources: Arc<dyn ResourceManager>,
    policy: std::sync::RwLock<crate::decision::Policy>,
    gpu_policy: std::sync::RwLock<GpuPolicy>,
}

impl DefaultScheduler {
    pub fn new(resources: Arc<dyn ResourceManager>) -> Self {
        Self {
            resources,
            policy: std::sync::RwLock::new(crate::decision::Policy::Balanced),
            gpu_policy: std::sync::RwLock::new(GpuPolicy::Adaptive),
        }
    }

    pub fn get_split_decision(&self, len: usize, state: &HardwareState) -> (usize, usize) {
        let policy = *self.policy.read().unwrap();
        let gpu_policy = *self.gpu_policy.read().unwrap();

        match gpu_policy {
            GpuPolicy::ForceCpu => (0, len),
            GpuPolicy::ForceGpu => (len, 0),
            GpuPolicy::Adaptive => {
                if len < 100_000 || state.gpu.is_none() {
                    (0, len)
                } else {
                    match policy {
                        crate::decision::Policy::Performance => {
                            let gpu_share = (len as f32 * 0.9) as usize;
                            (gpu_share, len - gpu_share)
                        }
                        crate::decision::Policy::Balanced => {
                            let gpu_share = (len as f32 * 0.7) as usize;
                            (gpu_share, len - gpu_share)
                        }
                        _ => (0, len),
                    }
                }
            }
        }
    }
}

impl UnifiedScheduler for DefaultScheduler {
    fn prepare_task(&self) -> CoreResult<()> {
        if self.resources.reserve_threads(1) {
            Ok(())
        } else {
            Err(ArchXCoreError::ResourceError("Failed to reserve threads".to_string()))
        }
    }

    fn complete_task(&self) {
        self.resources.release_threads(1);
    }

    fn set_policy(&self, policy: crate::decision::Policy) {
        if let Ok(mut p) = self.policy.write() {
            *p = policy;
        }
    }

    fn set_gpu_policy(&self, policy: GpuPolicy) {
        if let Ok(mut p) = self.gpu_policy.write() {
            *p = policy;
        }
    }
}
