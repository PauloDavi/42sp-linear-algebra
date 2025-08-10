//! # Cross Product
//!
//! This module provides the implementation of cross product for three-dimensional vectors.

use std::ops::{Mul, Sub};

use crate::Vector;

/// Calculates the cross product (vector product) between two three-dimensional vectors.
///
/// The cross product of two vectors **u** and **v** results in a third vector
/// perpendicular to both, with magnitude equal to the area of the parallelogram
/// formed by the two vectors.
///
/// For vectors **u** = [u₁, u₂, u₃] and **v** = [v₁, v₂, v₃], the cross product is:
/// **u × v** = [u₂v₃ - u₃v₂, u₃v₁ - u₁v₃, u₁v₂ - u₂v₁]
///
/// # Arguments
///
/// * `u` - The first vector (must have exactly 3 elements)
/// * `v` - The second vector (must have exactly 3 elements)
///
/// # Returns
///
/// A vector perpendicular to the input vectors. If either vector does not have
/// exactly 3 elements, returns a copy of the first vector.
///
/// # Examples
///
/// ```rust
/// use linear_algebra::{Vector, cross_product};
///
/// // Unit vectors of x and y axes
/// let i = Vector::from([1.0, 0.0, 0.0]);
/// let j = Vector::from([0.0, 1.0, 0.0]);
///
/// // The cross product i × j should result in vector k
/// let k = cross_product(&i, &j);
/// assert_eq!(k, Vector::from([0.0, 0.0, 1.0]));
///
/// // Cross product of parallel vectors is zero
/// let u = Vector::from([2.0, 0.0, 0.0]);
/// let v = Vector::from([4.0, 0.0, 0.0]);
/// let result = cross_product(&u, &v);
/// assert_eq!(result, Vector::from([0.0, 0.0, 0.0]));
/// ```
///
/// # Properties
///
/// - Anticommutative: **u × v** = -(**v × u**)
/// - Distributive: **u × (v + w)** = **u × v** + **u × w**
/// - The result is perpendicular to both input vectors
/// - The magnitude is |**u**| × |**v**| × sin(θ), where θ is the angle between vectors
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
