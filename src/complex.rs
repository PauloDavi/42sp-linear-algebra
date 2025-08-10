use crate::traits::{Conjugate, Magnitude, Negative, One, Zero};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Represents a complex number in the form `a + bi`.
///
/// The `Complex` structure implements all basic arithmetic operations
/// for complex numbers, including conjugation and magnitude calculation.
///
/// # Examples
///
/// ```rust
/// use linear_algebra_42::{Complex, traits::Magnitude};
///
/// // Creating complex numbers
/// let z1 = Complex::new(3.0, 4.0);  // 3 + 4i
/// let z2 = Complex::new(1.0, 2.0);  // 1 + 2i
///
/// // Basic operations
/// let sum = z1 + z2;               // (4 + 6i)
/// let product = z1 * z2;           // (-5 + 10i)
/// let conjugate = z1.conjugate();  // (3 - 4i)
/// let magnitude = z1.magnitude();  // 5.0
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    r: f32,
    i: f32,
}

impl Complex {
    /// Creates a new complex number with the specified real and imaginary parts.
    ///
    /// # Arguments
    ///
    /// * `real` - The real part of the complex number
    /// * `imaginary` - The imaginary part of the complex number
    ///
    /// # Examples
    ///
    /// ```rust
    /// use linear_algebra_42::Complex;
    ///
    /// let z = Complex::new(3.0, 4.0);
    /// assert_eq!(z.real(), 3.0);
    /// assert_eq!(z.imaginary(), 4.0);
    /// ```
    pub fn new(real: f32, imaginary: f32) -> Self {
        Complex {
            r: real,
            i: imaginary,
        }
    }

    /// Returns the real part of the complex number.
    pub fn real(&self) -> f32 {
        self.r
    }

    /// Returns the imaginary part of the complex number.
    pub fn imaginary(&self) -> f32 {
        self.i
    }

    /// Returns the complex conjugate (imaginary part with sign flipped).
    ///
    /// For a complex number `z = a + bi`, the conjugate is `z* = a - bi`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use linear_algebra_42::Complex;
    ///
    /// let z = Complex::new(3.0, 4.0);
    /// let conj = z.conjugate();
    /// assert_eq!(conj, Complex::new(3.0, -4.0));
    /// ```
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

impl Conjugate for Complex {
    fn conjugate(&self) -> Self {
        Complex {
            r: self.r,
            i: -self.i,
        }
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
