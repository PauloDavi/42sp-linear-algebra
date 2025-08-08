use std::ops::{Add, Mul};

use crate::errors::InterpolationError;

pub fn lerp<V>(u: V, v: V, t: f32) -> Result<V, InterpolationError>
where
    V: Add<Output = V> + Mul<f32, Output = V>,
{
    if t < 0.0 || t > 1.0 {
        return Err(InterpolationError::InvalidParameterT { t });
    }

    let u_scaled = u * (1.0 - t);
    let v_scaled = v * t;
    Ok(u_scaled + v_scaled)
}
