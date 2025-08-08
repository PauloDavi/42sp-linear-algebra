use std::ops::{Add, Mul};

use crate::{errors::LinearCombinationError, vector::Vector};

pub fn linear_combination<K>(
    vectors: &[Vector<K>],
    coefficients: &[K],
) -> Result<Vector<K>, LinearCombinationError>
where
    K: Clone + Default + Copy + Add<Output = K> + Mul<Output = K>,
{
    if vectors.len() != coefficients.len() {
        return Err(LinearCombinationError::CoefficientsDimensionMismatch {
            coefficients: coefficients.len(),
            vectors: vectors.len(),
        });
    }

    let len = match vectors.first() {
        Some(first) => first.len(),
        None => return Ok(Vector::from([])),
    };

    for vector in vectors {
        if vector.len() != len {
            return Err(LinearCombinationError::VectorsDimensionMismatch {
                expected_len: len,
                founded_len: vector.len(),
            });
        }
    }

    let mut result = Vector::with_default(len);
    for (vector, &coefficient) in vectors.iter().zip(coefficients.iter()) {
        let scaled_vector = vector.scl_new(coefficient);
        for (i, &value) in scaled_vector.iter().enumerate() {
            result[i] = result[i] + value;
        }
    }

    Ok(result)
}
