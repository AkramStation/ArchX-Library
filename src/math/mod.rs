use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ArithmeticResult<T> {
    pub value: T,
    pub overflowed: bool,
}

pub trait SafeMath<T> {
    fn safe_add(a: T, b: T) -> ArithmeticResult<T>;
    fn safe_mul(a: T, b: T) -> ArithmeticResult<T>;
}

impl SafeMath<i32> for i32 {
    fn safe_add(a: i32, b: i32) -> ArithmeticResult<i32> {
        let (val, overflow) = a.overflowing_add(b);
        ArithmeticResult { value: val, overflowed: overflow }
    }
    
    fn safe_mul(a: i32, b: i32) -> ArithmeticResult<i32> {
        let (val, overflow) = a.overflowing_mul(b);
        ArithmeticResult { value: val, overflowed: overflow }
    }
}

impl SafeMath<f32> for f32 {
    fn safe_add(a: f32, b: f32) -> ArithmeticResult<f32> {
        let val = a + b;
        ArithmeticResult { value: val, overflowed: val.is_infinite() }
    }
    
    fn safe_mul(a: f32, b: f32) -> ArithmeticResult<f32> {
        let val = a * b;
        ArithmeticResult { value: val, overflowed: val.is_infinite() }
    }
}
