use std::ops::{Mul, Sub};

use crate::Vector;

pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    K: Copy + Sub<Output = K> + Mul<Output = K>,
{
    if u.len() != 3 || v.len() != 3 {
        return Vector::from(u.as_slice());
    }

    Vector::from([
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ])
}
