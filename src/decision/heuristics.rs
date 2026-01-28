use crate::detect::HardwareState;
use crate::decision::policy::Policy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStrategy {
    Sequential,
    Parallel(usize),
    Chunked { threads: usize, chunk_size: usize },
    Throttled,
    AsyncAdaptive,
}

pub struct Heuristics;

impl Heuristics {
    pub fn decide(state: &HardwareState, policy: Policy) -> ExecutionStrategy {
        // High load protection
        if state.cpu.usage > 90.0 || policy == Policy::ProtectDevice {
            return ExecutionStrategy::Throttled;
        }

        match policy {
            Policy::Performance => {
                ExecutionStrategy::Parallel(state.cpu.logical_threads)
            }
            Policy::PowerSaving => {
                ExecutionStrategy::Sequential
            }
            Policy::Balanced => {
                if state.cpu.logical_threads > 4 {
                    ExecutionStrategy::Parallel(state.cpu.logical_threads / 2)
                } else {
                    ExecutionStrategy::Sequential
                }
            }
            Policy::ProtectDevice => ExecutionStrategy::Throttled,
            Policy::SmartAuto => {
                if state.cpu.usage > 50.0 {
                    ExecutionStrategy::Parallel(state.cpu.logical_threads / 2)
                } else {
                    ExecutionStrategy::Parallel(state.cpu.logical_threads)
                }
            }
        }
    }
}
