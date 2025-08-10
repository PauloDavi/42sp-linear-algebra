use std::ops::{Add, Mul};

use crate::{
    Vector,
    traits::{Magnitude, Zero},
};

pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> f32
where
    K: Copy + Zero + Add<Output = K> + Mul<Output = K> + Magnitude<Output = f32> + Into<f32>,
{
    debug_assert_eq!(u.len(), v.len(), "Vectors must have same dimension");

    let dot_product = u.dot(v).into();
    let norm_u = u.norm();
    let norm_v = v.norm();

    if norm_u == 0.0 || norm_v == 0.0 {
        return 0.0;
    }

    dot_product / (norm_u * norm_v)
}
