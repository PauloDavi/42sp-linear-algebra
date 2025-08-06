use std::ops::{Index, IndexMut};

use crate::{errors::VectorError, types::Scalar};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<K> {
    data: Vec<K>,
}

impl<K> Vector<K>
where
    K: Scalar,
{
    pub fn new(data: &[K]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    pub fn with_default(len: usize) -> Self {
        let data = vec![K::default(); len];
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.data.iter()
    }

    pub fn add(&mut self, other: &Vector<K>) -> Result<(), VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::DimensionMismatch);
        }

        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, &b)| *a += b);

        Ok(())
    }

    pub fn sub(&mut self, other: &Vector<K>) -> Result<(), VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::DimensionMismatch);
        }

        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, &b)| *a -= b);

        Ok(())
    }

    pub fn scalar(&mut self, a: K) {
        self.data.iter_mut().for_each(|element| *element *= a);
    }

    pub fn add_new(&self, other: &Vector<K>) -> Result<Vector<K>, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::DimensionMismatch);
        }

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();

        Ok(Vector { data })
    }

    pub fn sub_new(&self, other: &Vector<K>) -> Result<Vector<K>, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::DimensionMismatch);
        }

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a - b)
            .collect();

        Ok(Vector { data })
    }

    pub fn scalar_new(&self, a: K) -> Vector<K> {
        let data: Vec<K> = self.data.iter().map(|&element| element * a).collect();
        Vector { data }
    }
}

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
