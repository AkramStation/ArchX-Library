use crate::detect::HardwareState;

pub struct Sampler {
    pub history: Vec<HardwareState>,
    pub max_samples: usize,
}

impl Sampler {
    pub fn new(max_samples: usize) -> Self {
        Self {
            history: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn sample(&mut self) {
        if self.history.len() >= self.max_samples {
            self.history.remove(0);
        }
        self.history.push(HardwareState::capture());
    }

    pub fn current_load(&self) -> f32 {
        self.history.last().map(|s| s.cpu.usage).unwrap_or(0.0)
    }
}
