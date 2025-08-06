use std::ops::{Index, IndexMut};

use crate::{errors::MatrixError, types::Scalar};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<K> {
    rows: usize,
    columns: usize,
    data: Vec<Vec<K>>,
}

impl<K> Matrix<K>
where
    K: Scalar,
{
    pub fn new(data: &[&[K]]) -> Result<Self, MatrixError> {
        let rows = data.len();
        let columns = if rows == 0 { 0 } else { data[0].len() };

        if !data.iter().all(|row| row.len() == columns) {
            return Err(MatrixError::RowsLengthMismatch);
        }

        Ok(Self {
            rows,
            columns,
            data: data.iter().map(|item| item.to_vec()).collect(),
        })
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    pub fn add(&mut self, other: &Matrix<K>) -> Result<(), MatrixError> {
        if self.shape() != other.shape() {
            return Err(MatrixError::DimensionMismatch);
        }

        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| {
                a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a += *b);
            });

        Ok(())
    }

    pub fn sub(&mut self, other: &Matrix<K>) -> Result<(), MatrixError> {
        if self.shape() != other.shape() {
            return Err(MatrixError::DimensionMismatch);
        }

        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| {
                a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a -= *b);
            });
        Ok(())
    }

    pub fn scalar(&mut self, a: K) {
        self.data.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|element| *element *= a);
        });
    }

    pub fn add_new(&self, other: &Matrix<K>) -> Result<Matrix<K>, MatrixError> {
        if self.shape() != other.shape() {
            return Err(MatrixError::DimensionMismatch);
        }

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.iter().zip(b.iter()).map(|(&a, &b)| a + b).collect())
            .collect();

        Ok(Matrix {
            rows: self.rows,
            columns: self.columns,
            data,
        })
    }

    pub fn sub_new(&self, other: &Matrix<K>) -> Result<Matrix<K>, MatrixError> {
        if self.shape() != other.shape() {
            return Err(MatrixError::DimensionMismatch);
        }

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.iter().zip(b.iter()).map(|(&a, &b)| a - b).collect())
            .collect();

        Ok(Matrix {
            rows: self.rows,
            columns: self.columns,
            data,
        })
    }

    pub fn scalar_new(&self, a: K) -> Matrix<K> {
        let data = self
            .data
            .iter()
            .map(|rows| rows.iter().map(|&element| element * a).collect())
            .collect();

        Matrix {
            rows: self.rows,
            columns: self.columns,
            data,
        }
    }
}

impl<K> Index<usize> for Matrix<K> {
    type Output = [K];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K> IndexMut<usize> for Matrix<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
