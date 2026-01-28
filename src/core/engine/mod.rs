use crate::core::error::CoreResult;
use crate::math::MathMode;

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticMode {
    Safe,
    Fast,
    Balanced,
}

impl From<MathMode> for ArithmeticMode {
    fn from(mode: MathMode) -> Self {
        match mode {
            MathMode::Safe => ArithmeticMode::Safe,
            MathMode::Fast => ArithmeticMode::Fast,
            MathMode::Balanced => ArithmeticMode::Balanced,
        }
    }
}

pub trait MathProcessor: Send + Sync {
    fn add(&self, a: &[f32], b: &[f32], out: &mut [f32], mode: ArithmeticMode) -> CoreResult<()>;
    fn sub(&self, a: &[f32], b: &[f32], out: &mut [f32], mode: ArithmeticMode) -> CoreResult<()>;
    fn mul(&self, a: &[f32], b: &[f32], out: &mut [f32], mode: ArithmeticMode) -> CoreResult<()>;
    fn dot(&self, a: &[f32], b: &[f32], mode: ArithmeticMode) -> CoreResult<f32>;
    fn sum(&self, a: &[f32], mode: ArithmeticMode) -> CoreResult<f32>;
}

pub struct DefaultMathProcessor;

use crate::math::{AdaptiveMath, MathMode as LegacyMathMode};

impl DefaultMathProcessor {
    fn to_legacy_mode(mode: ArithmeticMode) -> LegacyMathMode {
        match mode {
            ArithmeticMode::Safe => LegacyMathMode::Safe,
            ArithmeticMode::Fast => LegacyMathMode::Fast,
            ArithmeticMode::Balanced => LegacyMathMode::Balanced,
        }
    }
}

impl MathProcessor for DefaultMathProcessor {
    fn add(&self, a: &[f32], b: &[f32], out: &mut [f32], mode: ArithmeticMode) -> CoreResult<()> {
        let legacy_mode = Self::to_legacy_mode(mode);
        // In a full implementation, we would use SIMD kernels that respect 'mode'
        // For now, we simulate by applying AdaptiveMath logic or using parallel dispatch
        for i in 0..a.len() {
            out[i] = f32::compute_add(a[i], b[i], legacy_mode);
        }
        Ok(())
    }

    fn sub(&self, a: &[f32], b: &[f32], out: &mut [f32], mode: ArithmeticMode) -> CoreResult<()> {
        let legacy_mode = Self::to_legacy_mode(mode);
        for i in 0..a.len() {
            out[i] = f32::compute_sub(a[i], b[i], legacy_mode);
        }
        Ok(())
    }

    fn mul(&self, a: &[f32], b: &[f32], out: &mut [f32], mode: ArithmeticMode) -> CoreResult<()> {
        let legacy_mode = Self::to_legacy_mode(mode);
        for i in 0..a.len() {
            out[i] = f32::compute_mul(a[i], b[i], legacy_mode);
        }
        Ok(())
    }

    fn dot(&self, a: &[f32], b: &[f32], _mode: ArithmeticMode) -> CoreResult<f32> {
        // Dot product typically doesn't have saturating/wrapping variants for floats in the same way
        Ok(crate::runtime::ArchXSched::parallel_dot(a, b))
    }

    fn sum(&self, a: &[f32], _mode: ArithmeticMode) -> CoreResult<f32> {
        Ok(crate::runtime::ArchXSched::parallel_sum(a))
    }
}
