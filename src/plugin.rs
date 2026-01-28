use std::sync::{OnceLock, RwLock};

/// A trait for custom compute implementations that can be plugged into ArchX.
/// 
/// WHY: v0.7 introduces a plugin system to allow users to add their own 
/// specialized implementations (e.g., custom AVX-512 kernels or DSP offloads).
pub trait ComputePlugin: Send + Sync {
    /// Returns the name of the plugin.
    fn name(&self) -> &str;
    /// The actual add implementation.
    fn add(&self, a: &[f32], b: &[f32], out: &mut [f32]) -> bool;
}

static PLUGINS: OnceLock<RwLock<Vec<Box<dyn ComputePlugin>>>> = OnceLock::new();

/// Registers a new compute plugin.
pub fn register_plugin(plugin: Box<dyn ComputePlugin>) {
    let registry = PLUGINS.get_or_init(|| RwLock::new(Vec::new()));
    if let Ok(mut lock) = registry.write() {
        lock.push(plugin);
    }
}

/// Tries to execute the operation using registered plugins.
pub(crate) fn try_plugins(a: &[f32], b: &[f32], out: &mut [f32]) -> bool {
    if let Some(registry) = PLUGINS.get() {
        if let Ok(lock) = registry.read() {
            for plugin in lock.iter() {
                if plugin.add(a, b, out) {
                    return true;
                }
            }
        }
    }
    false
}
