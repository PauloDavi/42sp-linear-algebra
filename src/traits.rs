//! # Fundamental Traits for Linear Algebra
//!
//! This module defines essential traits that enable generic operations
//! over different numeric types in the linear algebra library.

/// Trait for types that have an additive zero element.
///
/// This trait defines the additive identity and allows checking
/// whether a value equals zero.
pub trait Zero {
    /// Returns the zero element of the type.
    fn zero() -> Self;

    /// Checks if the value equals the zero element.
    fn is_zero(&self) -> bool;
}

/// Trait for types that have a multiplicative one element.
///
/// This trait defines the multiplicative identity.
pub trait One {
    /// Returns the one element of the type.
    fn one() -> Self;
}

/// Trait for types that have a negative one element.
///
/// This trait is useful for defining the multiplicative opposite element.
pub trait Negative {
    /// Returns the negative one element of the type.
    fn negative_one() -> Self;
}

/// Trait for types that have a defined magnitude or norm.
///
/// This trait allows calculating the magnitude (modulus) of a value,
/// essential for linear algebra operations like vector norms.
pub trait Magnitude {
    /// The return type of the magnitude.
    type Output;

    /// Calculates the magnitude of the value.
    fn magnitude(&self) -> Self::Output;
}

/// Trait for types that support conjugation operation.
///
/// This trait is fundamental for working with complex numbers
/// and operations that require complex conjugation, such as
/// inner products in complex spaces.
///
/// # Examples
///
/// For real numbers, the conjugate is the number itself:
/// ```rust
/// use linear_algebra::traits::Conjugate;
///
/// assert_eq!(5.0_f32.conjugate(), 5.0);
/// ```
///
/// For complex numbers, the conjugate changes the sign of the imaginary part:
/// ```rust
/// use linear_algebra::{Complex, traits::Conjugate};
///
/// let z = Complex::new(3.0, 4.0);
/// let conj = z.conjugate();
/// assert_eq!(conj, Complex::new(3.0, -4.0));
/// ```
pub trait Conjugate {
    /// Returns the conjugate of the value.
    fn conjugate(&self) -> Self;
}

impl Zero for i8 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for i8 {
    fn one() -> Self {
        1
    }
}

impl Negative for i8 {
    fn negative_one() -> Self {
        -1
    }
}

impl Zero for i16 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for i16 {
    fn one() -> Self {
        1
    }
}

impl Negative for i16 {
    fn negative_one() -> Self {
        -1
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}

impl Negative for i32 {
    fn negative_one() -> Self {
        -1
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for i64 {
    fn one() -> Self {
        1
    }
}

impl Negative for i64 {
    fn negative_one() -> Self {
        -1
    }
}

impl Zero for u8 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for u8 {
    fn one() -> Self {
        1
    }
}

impl Zero for u16 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for u16 {
    fn one() -> Self {
        1
    }
}

impl Zero for u32 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for u32 {
    fn one() -> Self {
        1
    }
}

impl Zero for u64 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for u64 {
    fn one() -> Self {
        1
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

impl Negative for f32 {
    fn negative_one() -> Self {
        -1.0
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}

impl Negative for f64 {
    fn negative_one() -> Self {
        -1.0
    }
}

impl Magnitude for i8 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (*self as f32).abs()
    }
}

impl Magnitude for i16 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (*self as f32).abs()
    }
}

impl Magnitude for i32 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (*self as f32).abs()
    }
}

impl Magnitude for i64 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (*self as f32).abs()
    }
}

impl Magnitude for u8 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        *self as f32
    }
}

impl Magnitude for u16 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        *self as f32
    }
}

impl Magnitude for u32 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        *self as f32
    }
}

impl Magnitude for u64 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        *self as f32
    }
}

impl Magnitude for f32 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        self.abs()
    }
}

impl Magnitude for f64 {
    type Output = f32;

    fn magnitude(&self) -> Self::Output {
        (*self as f32).abs()
    }
}

impl Conjugate for i8 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for i16 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for i32 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for i64 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for u8 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for u16 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for u32 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for u64 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for f32 {
    fn conjugate(&self) -> Self {
        *self
    }
}

impl Conjugate for f64 {
    fn conjugate(&self) -> Self {
        *self
    }
}
