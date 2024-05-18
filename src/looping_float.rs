use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Copy, Clone)]
pub struct LoopingFloat<const MAX: u32> {
    value: f32,
}

impl<const MAX: u32> LoopingFloat<MAX> {
    pub fn new(value: f32) -> Self {
        let mut lf = LoopingFloat { value };
        lf.wrap();
        lf
    }

    fn wrap(&mut self) {
        let max = MAX as f32;
        if self.value >= max {
            self.value = self.value % max;
        } else if self.value < 0.0 {
            self.value = max + (self.value % max);
        }
    }

    pub fn get(&self) -> f32 {
        self.value
    }

    pub fn to_f32(self) -> f32 {
        self.value
    }

    pub fn difference(&self, other: f32) -> f32 {
        let max = MAX as f32;
        let delta = other - self.value;
        let wrapped_delta = if delta > 0.0 {
            delta - max
        } else {
            delta + max
        };

        if delta.abs() < wrapped_delta.abs() {
            delta
        } else {
            wrapped_delta
        }
    }
}

impl<const MAX: u32> Add<f32> for LoopingFloat<MAX> {
    type Output = Self;

    fn add(mut self, other: f32) -> Self::Output {
        self.value += other;
        self.wrap();
        self
    }
}

impl<const MAX: u32> Sub<f32> for LoopingFloat<MAX> {
    type Output = Self;

    fn sub(mut self, other: f32) -> Self::Output {
        self.value -= other;
        self.wrap();
        self
    }
}

impl<const MAX: u32> Mul<f32> for LoopingFloat<MAX> {
    type Output = Self;

    fn mul(mut self, other: f32) -> Self::Output {
        self.value *= other;
        self.wrap();
        self
    }
}

impl<const MAX: u32> Div<f32> for LoopingFloat<MAX> {
    type Output = Self;

    fn div(mut self, other: f32) -> Self::Output {
        self.value /= other;
        self.wrap();
        self
    }
}

impl<const MAX: u32> AddAssign<f32> for LoopingFloat<MAX> {
    fn add_assign(&mut self, other: f32) {
        self.value += other;
        self.wrap();
    }
}

impl<const MAX: u32> SubAssign<f32> for LoopingFloat<MAX> {
    fn sub_assign(&mut self, other: f32) {
        self.value -= other;
        self.wrap();
    }
}

impl<const MAX: u32> MulAssign<f32> for LoopingFloat<MAX> {
    fn mul_assign(&mut self, other: f32) {
        self.value *= other;
        self.wrap();
    }
}

impl<const MAX: u32> DivAssign<f32> for LoopingFloat<MAX> {
    fn div_assign(&mut self, other: f32) {
        self.value /= other;
        self.wrap();
    }
}

impl<const MAX: u32> fmt::Display for LoopingFloat<MAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}