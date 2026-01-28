use crate::detect::HardwareState;
use crate::decision::{Heuristics, ExecutionStrategy, Policy};

pub struct AdaptiveEngine;

impl AdaptiveEngine {
    pub fn decide(state: &HardwareState, policy: Policy) -> ExecutionStrategy {
        Heuristics::decide(state, policy)
    }
}
