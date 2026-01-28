pub mod error;
pub mod detect;
pub mod scheduler;
pub mod engine;
pub mod resource;
pub mod profiler;

pub use error::{ArchXCoreError, CoreResult};

use std::sync::Arc;

/// The central orchestrator for the ArchX Core Engine.
pub struct CoreEngine {
    pub(crate) hardware: Arc<dyn detect::HardwareProvider>,
    pub(crate) scheduler: Arc<dyn scheduler::UnifiedScheduler>,
    pub(crate) engine: Arc<dyn engine::MathProcessor>,
    pub(crate) resources: Arc<dyn resource::ResourceManager>,
}

impl CoreEngine {
    pub fn new() -> Self {
        let resources = Arc::new(resource::DefaultResourceManager::new());
        let hardware = Arc::new(detect::DefaultHardwareProvider);
        let scheduler = Arc::new(scheduler::DefaultScheduler::new(resources.clone()));
        let engine = Arc::new(engine::DefaultMathProcessor);

        Self {
            hardware,
            scheduler,
            engine,
            resources,
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: std::sync::OnceLock<CoreEngine> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(CoreEngine::new)
    }

    pub fn resources(&self) -> &dyn resource::ResourceManager {
        self.resources.as_ref()
    }
}

pub use detect::{HardwareProvider, HardwareState};
pub use scheduler::UnifiedScheduler;
pub use engine::{MathProcessor, ArithmeticMode};
pub use resource::ResourceManager;
