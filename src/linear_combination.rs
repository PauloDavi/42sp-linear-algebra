use crate::{errors::VectorError, types::Scalar, vector::Vector};

pub fn linear_combination<K>(
    vectors: &[Vector<K>],
    coefficients: &[K],
) -> Result<Vector<K>, VectorError>
where
    K: Scalar,
{
    if vectors.len() != coefficients.len() {
        return Err(VectorError::LinearCombinationDimensionMismatch);
    }

    let len = match vectors.first() {
        Some(first) => first.len(),
        None => return Ok(Vector::new(&[])),
    };

    for vector in vectors {
        if vector.len() != len {
            return Err(VectorError::DimensionMismatch);
        }
    }

    let mut result = Vector::with_default(len);
    for (vector, &coefficient) in vectors.iter().zip(coefficients.iter()) {
        let scaled_vector = vector.scalar_new(coefficient);
        for (i, &value) in scaled_vector.iter().enumerate() {
            result[i] += value;
        }
    }

    Ok(result)
}
