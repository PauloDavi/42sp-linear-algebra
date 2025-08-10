use std::ops::{Add, Mul};

use crate::{errors::LinearCombinationError, traits::Zero, vector::Vector};

pub fn linear_combination<K, const N: usize>(
    vectors: [Vector<K>; N],
    coefficients: [K; N],
) -> Result<Vector<K>, LinearCombinationError>
where
    K: Copy + Zero + Add<Output = K> + Mul<Output = K>,
{
    if N == 0 {
        return Ok(Vector::from(Vec::new()));
    }

    let len = vectors[0].len();
    for vector in vectors.iter() {
        if vector.len() != len {
            return Err(LinearCombinationError::VectorsDimensionMismatch {
                expected_len: len,
                founded_len: vector.len(),
            });
        }
    }

    let mut result_data = vec![K::zero(); len];
    for (vector, &coefficient) in vectors.iter().zip(coefficients.iter()) {
        for (i, &value) in vector.as_slice().iter().enumerate() {
            result_data[i] = result_data[i] + (value * coefficient);
        }
    }

    Ok(Vector::from(result_data))
}
