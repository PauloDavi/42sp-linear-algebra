use crate::{Magnitude, Negative, One, Zero};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    r: f32,
    i: f32,
}

impl Complex {
    pub fn new(real: f32, imaginary: f32) -> Self {
        Complex {
            r: real,
            i: imaginary,
        }
    }

    pub fn real(&self) -> f32 {
        self.r
    }

    pub fn imaginary(&self) -> f32 {
        self.i
    }

    pub fn conjugate(&self) -> Self {
        Complex {
            r: self.r,
            i: -self.i,
        }
    }
}

impl Zero for Complex {
    fn zero() -> Self {
        Complex { r: 0., i: 0. }
    }

    fn is_zero(&self) -> bool {
        self.r == 0. && self.i == 0.
    }
}

impl One for Complex {
    fn one() -> Self {
        Complex { r: 1., i: 0. }
    }
}

impl Negative for Complex {
    fn negative_one() -> Self {
        Complex { r: -1., i: 0. }
    }
}

impl Magnitude for Complex {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (self.r.powi(2) + self.i.powi(2)).sqrt()
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Complex {
            r: self.r - other.r,
            i: self.i - other.i,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Complex {
            r: self.r * other.r - self.i * other.i,
            i: self.r * other.i + self.i * other.r,
        }
    }
}

impl Mul<f32> for Complex {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Complex {
            r: self.r * scalar,
            i: self.i * scalar,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let denominator = other.r * other.r + other.i * other.i;
        Complex {
            r: (self.r * other.r + self.i * other.i) / denominator,
            i: (self.i * other.r - self.r * other.i) / denominator,
        }
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Complex {
            r: -self.r,
            i: -self.i,
        }
    }
}

impl PartialOrd for Complex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.magnitude().partial_cmp(&other.magnitude())
    }
}

impl From<Complex> for f32 {
    fn from(val: Complex) -> Self {
        val.r
    }
}
