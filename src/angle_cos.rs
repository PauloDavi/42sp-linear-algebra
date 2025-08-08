use std::ops::{Add, Mul};

use crate::Vector;

pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> f32
where
    K: Copy + Default + Add<Output = K> + Mul<Output = K> + Into<f32>,
{
    u.dot(v).into() / (u.norm() * v.norm())
}
