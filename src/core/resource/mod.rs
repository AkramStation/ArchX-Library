use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Serialize, Default)]
pub struct UsageInfo {
    pub active_threads: usize,
    pub gpu_memory_used_kb: usize,
    pub active_tasks: usize,
}

pub trait ResourceManager: Send + Sync {
    fn track_usage(&self, info: UsageInfo);
    fn get_current_usage(&self) -> UsageInfo;
    fn reserve_threads(&self, count: usize) -> bool;
    fn release_threads(&self, count: usize);
}

pub struct DefaultResourceManager {
    active_threads: AtomicUsize,
    active_tasks: AtomicUsize,
    // Add more tracking such as GPU memory if platform supported
}

impl DefaultResourceManager {
    pub fn new() -> Self {
        Self {
            active_threads: AtomicUsize::new(0),
            active_tasks: AtomicUsize::new(0),
        }
    }
}

impl ResourceManager for DefaultResourceManager {
    fn track_usage(&self, info: UsageInfo) {
        // Simple update logic for demo/v3.0 start
        self.active_threads.store(info.active_threads, Ordering::Relaxed);
        self.active_tasks.store(info.active_tasks, Ordering::Relaxed);
    }

    fn get_current_usage(&self) -> UsageInfo {
        UsageInfo {
            active_threads: self.active_threads.load(Ordering::Relaxed),
            gpu_memory_used_kb: 0, // TODO: Interop with GPU backend
            active_tasks: self.active_tasks.load(Ordering::Relaxed),
        }
    }

    fn reserve_threads(&self, count: usize) -> bool {
        // Rudimentary admission control
        let current = self.active_threads.load(Ordering::SeqCst);
        let max = rayon::current_num_threads();
        if current + count > max {
            return false;
        }
        self.active_threads.fetch_add(count, Ordering::SeqCst);
        true
    }

    fn release_threads(&self, count: usize) {
        self.active_threads.fetch_sub(count, Ordering::SeqCst);
    }
}
