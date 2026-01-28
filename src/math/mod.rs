use serde::{Serialize, Deserialize};

/// Defines the arithmetic safety and performance strategy for computations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathMode {
    /// Full checked arithmetic. Detects overflows and infinities, 
    /// returning errors via `ArithmeticResult`. Safest but slowest.
    Safe,
    /// Standard wrapping arithmetic. Optimized for maximum raw speed 
    /// where overflow is either impossible or acceptable.
    Fast,
    /// Saturating arithmetic. Results are clamped to the minimum/maximum 
    /// values of the type on overflow. Ideal for stable signal processing.
    Balanced,
}

/// A result wrapper for overflow-protected arithmetic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArithmeticResult<T> {
    /// The computed result value.
    pub value: T,
    /// Indicates if an overflow or infinity occurred during calculation.
    pub overflowed: bool,
}

/// Trait for overflow-protected primitive operations.
pub trait SafeMath<T> {
    /// Performs addition with overflow detection.
    fn safe_add(a: T, b: T) -> ArithmeticResult<T>;
    /// Performs subtraction with overflow detection.
    fn safe_sub(a: T, b: T) -> ArithmeticResult<T>;
    /// Performs multiplication with overflow detection.
    fn safe_mul(a: T, b: T) -> ArithmeticResult<T>;
    /// Performs division with div-by-zero and overflow detection.
    fn safe_div(a: T, b: T) -> ArithmeticResult<T>;
}

/// Trait for mode-aware adaptive arithmetic.
pub trait AdaptiveMath<T> {
    fn compute_add(a: T, b: T, mode: MathMode) -> T;
    fn compute_sub(a: T, b: T, mode: MathMode) -> T;
    fn compute_mul(a: T, b: T, mode: MathMode) -> T;
}

macro_rules! impl_safe_math_int {
    ($($t:ty),*) => {
        $(
            impl SafeMath<$t> for $t {
                fn safe_add(a: $t, b: $t) -> ArithmeticResult<$t> {
                    let (val, overflow) = a.overflowing_add(b);
                    ArithmeticResult { value: val, overflowed: overflow }
                }
                fn safe_sub(a: $t, b: $t) -> ArithmeticResult<$t> {
                    let (val, overflow) = a.overflowing_sub(b);
                    ArithmeticResult { value: val, overflowed: overflow }
                }
                fn safe_mul(a: $t, b: $t) -> ArithmeticResult<$t> {
                    let (val, overflow) = a.overflowing_mul(b);
                    ArithmeticResult { value: val, overflowed: overflow }
                }
                fn safe_div(a: $t, b: $t) -> ArithmeticResult<$t> {
                    if b == 0 {
                        return ArithmeticResult { value: 0 as $t, overflowed: true };
                    }
                    let (val, overflow) = a.overflowing_div(b);
                    ArithmeticResult { value: val, overflowed: overflow }
                }
            }

            impl AdaptiveMath<$t> for $t {
                fn compute_add(a: $t, b: $t, mode: MathMode) -> $t {
                    match mode {
                        MathMode::Safe => a.checked_add(b).unwrap_or(0 as $t),
                        MathMode::Fast => a.wrapping_add(b),
                        MathMode::Balanced => a.saturating_add(b),
                    }
                }
                fn compute_sub(a: $t, b: $t, mode: MathMode) -> $t {
                    match mode {
                        MathMode::Safe => a.checked_sub(b).unwrap_or(0 as $t),
                        MathMode::Fast => a.wrapping_sub(b),
                        MathMode::Balanced => a.saturating_sub(b),
                    }
                }
                fn compute_mul(a: $t, b: $t, mode: MathMode) -> $t {
                    match mode {
                        MathMode::Safe => a.checked_mul(b).unwrap_or(0 as $t),
                        MathMode::Fast => a.wrapping_mul(b),
                        MathMode::Balanced => a.saturating_mul(b),
                    }
                }
            }
        )*
    };
}

impl_safe_math_int!(i32, u32, i64, u64);

macro_rules! impl_safe_math_float {
    ($($t:ty),*) => {
        $(
            impl SafeMath<$t> for $t {
                fn safe_add(a: $t, b: $t) -> ArithmeticResult<$t> {
                    let val = a + b;
                    ArithmeticResult { value: val, overflowed: val.is_infinite() }
                }
                fn safe_sub(a: $t, b: $t) -> ArithmeticResult<$t> {
                    let val = a - b;
                    ArithmeticResult { value: val, overflowed: val.is_infinite() }
                }
                fn safe_mul(a: $t, b: $t) -> ArithmeticResult<$t> {
                    let val = a * b;
                    ArithmeticResult { value: val, overflowed: val.is_infinite() }
                }
                fn safe_div(a: $t, b: $t) -> ArithmeticResult<$t> {
                    let val = a / b;
                    ArithmeticResult { value: val, overflowed: val.is_infinite() || val.is_nan() }
                }
            }

            impl AdaptiveMath<$t> for $t {
                fn compute_add(a: $t, b: $t, _mode: MathMode) -> $t {
                    a + b // Floats don't have wrapping/saturating in the same way
                }
                fn compute_sub(a: $t, b: $t, _mode: MathMode) -> $t {
                    a - b
                }
                fn compute_mul(a: $t, b: $t, _mode: MathMode) -> $t {
                    a * b
                }
            }
        )*
    };
}

impl_safe_math_float!(f32, f64);
