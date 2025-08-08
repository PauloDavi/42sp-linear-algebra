use std::ops::{Add, Mul};

use crate::errors::InterpolationError;

pub fn lerp<K>(u: K, v: K, t: f32) -> Result<K, InterpolationError>
where
    K: Add<Output = K> + Mul<f32, Output = K>,
{
    if t < 0.0 || t > 1.0 {
        return Err(InterpolationError::InvalidParameterT { t });
    }

    let u_scaled = u * (1.0 - t);
    let v_scaled = v * t;
    Ok(u_scaled + v_scaled)
}
