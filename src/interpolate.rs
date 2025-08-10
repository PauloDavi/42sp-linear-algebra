//! # Linear Interpolation
//!
//! This module provides functions for linear interpolation between values.

use std::ops::{Add, Mul};

use crate::errors::InterpolationError;

/// Performs linear interpolation between two values.
///
/// Linear interpolation (LERP) calculates an intermediate value between two given
/// values `u` and `v` using the parameter `t`. The formula used is:
///
/// `result = u * (1 - t) + v * t`
///
/// # Arguments
///
/// * `u` - The initial value (when t = 0.0)
/// * `v` - The final value (when t = 1.0)
/// * `t` - The interpolation parameter (must be between 0.0 and 1.0)
///
/// # Returns
///
/// * `Ok(K)` - The interpolated value if t is in the range [0.0, 1.0]
/// * `Err(InterpolationError)` - If t is outside the valid range
///
/// # Examples
///
/// ```rust
/// use linear_algebra::lerp;
///
/// // Simple interpolation between two numbers
/// let result = lerp(0.0, 10.0, 0.5).unwrap();
/// assert_eq!(result, 5.0);
///
/// // Interpolation at the beginning (t = 0.0)
/// let result = lerp(0.0, 10.0, 0.0).unwrap();
/// assert_eq!(result, 0.0);
///
/// // Interpolation at the end (t = 1.0)
/// let result = lerp(0.0, 10.0, 1.0).unwrap();
/// assert_eq!(result, 10.0);
/// ```
///
/// # Errors
///
/// Returns `InterpolationError::InvalidParameterT` if the parameter `t`
/// is not in the range [0.0, 1.0].
pub fn lerp<K>(u: K, v: K, t: f32) -> Result<K, InterpolationError>
where
    K: Add<Output = K> + Mul<f32, Output = K>,
{
    if !(0.0..=1.0).contains(&t) {
        return Err(InterpolationError::InvalidParameterT { t });
    }

    Ok(u * (1.0 - t) + v * t)
}
