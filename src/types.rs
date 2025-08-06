use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

pub trait Scalar:
    Copy
    + Display
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Default
{
}

impl<T> Scalar for T where
    T: Copy
        + Display
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Default
{
}
