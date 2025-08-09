use std::ops::{Add, Mul};

use crate::errors::InterpolationError;

pub fn lerp<K>(u: K, v: K, t: f32) -> Result<K, InterpolationError>
where
    K: Add<Output = K> + Mul<f32, Output = K>,
{
    if !(0.0..=1.0).contains(&t) {
        return Err(InterpolationError::InvalidParameterT { t });
    }

    Ok(u * (1.0 - t) + v * t)
}
