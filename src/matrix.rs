use core::slice::{Iter, IterMut};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<K> {
    rows: usize,
    columns: usize,
    data: Vec<Vec<K>>,
}

impl<K> Matrix<K> {
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    pub fn iter(&self) -> Iter<'_, Vec<K>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Vec<K>> {
        self.data.iter_mut()
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.columns
    }
}

impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K>
where
    K: Clone,
{
    fn from(arr: [[K; C]; R]) -> Self {
        Self {
            rows: R,
            columns: C,
            data: arr.map(|row| row.to_vec()).to_vec(),
        }
    }
}

impl<K> Matrix<K>
where
    K: Clone + Copy,
{
    pub fn transpose(&self) -> Self {
        let mut transposed_data = vec![Vec::with_capacity(self.rows); self.columns];
        for row in &self.data {
            for (j, val) in row.iter().enumerate() {
                transposed_data[j].push(*val);
            }
        }
        Self {
            columns: self.rows,
            rows: self.columns,
            data: transposed_data,
        }
    }
}

impl<K> Matrix<K>
where
    K: Clone + Default,
{
    pub fn with_default(rows: usize, columns: usize) -> Self {
        Self {
            columns,
            rows,
            data: vec![vec![K::default(); columns]; rows],
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + Add<Output = K>,
{
    pub fn add(&mut self, other: &Self) {
        for (row_a, row_b) in self.data.iter_mut().zip(&other.data) {
            for (column_a, column_b) in row_a.iter_mut().zip(row_b) {
                *column_a = *column_a + *column_b;
            }
        }
    }

    pub fn add_new(&self, other: &Self) -> Self {
        Self {
            columns: self.columns,
            rows: self.rows,
            data: self
                .data
                .iter()
                .zip(&other.data)
                .map(|(row_a, row_b)| {
                    row_a
                        .iter()
                        .zip(row_b)
                        .map(|(&column_a, &column_b)| column_a + column_b)
                        .collect()
                })
                .collect(),
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + Sub<Output = K>,
{
    pub fn sub(&mut self, other: &Self) {
        for (row_a, row_b) in self.data.iter_mut().zip(&other.data) {
            for (column_a, column_b) in row_a.iter_mut().zip(row_b) {
                *column_a = *column_a - *column_b;
            }
        }
    }

    pub fn sub_new(&self, other: &Self) -> Self {
        Self {
            columns: self.columns,
            rows: self.rows,
            data: self
                .data
                .iter()
                .zip(&other.data)
                .map(|(row_a, row_b)| {
                    row_a
                        .iter()
                        .zip(row_b)
                        .map(|(&column_a, &column_b)| column_a - column_b)
                        .collect()
                })
                .collect(),
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + Mul<Output = K>,
{
    pub fn scl(&mut self, scalar: K) {
        for row in self.data.iter_mut() {
            for column in row.iter_mut() {
                *column = *column * scalar;
            }
        }
    }

    pub fn scl_new(&self, scalar: K) -> Self {
        Self {
            columns: self.columns,
            rows: self.rows,
            data: self
                .data
                .iter()
                .map(|row| row.iter().map(|column| *column * scalar).collect())
                .collect(),
        }
    }
}

impl<K> Add for Matrix<K>
where
    K: Copy + Add<Output = K>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self.add_new(&other)
    }
}

impl<K> Sub for Matrix<K>
where
    K: Copy + Sub<Output = K>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self.sub_new(&other)
    }
}

impl<K> Mul<K> for Matrix<K>
where
    K: Copy + Mul<Output = K>,
{
    type Output = Self;
    fn mul(self, scalar: K) -> Self::Output {
        self.scl_new(scalar)
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

impl<K> Matrix<K>
where
    K: Clone + Copy + Default + Add<Output = K> + Mul<Output = K>,
{
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let transposed = self.transpose();
        let mut data = vec![K::default(); self.rows];

        for i in 0..self.rows {
            data[i] = Vector::from(&transposed[i]).dot(vec);
        }

        Vector::from(data.as_slice())
    }

    pub fn mul_mat(&self, mat: &Self) -> Self {
        let transposed = mat.transpose();
        let mut data = vec![vec![K::default(); self.rows]; self.columns];

        for i in 0..self.rows {
            for j in 0..self.columns {
                data[i][j] = Vector::from(&transposed[j]).dot(&Vector::from(&self[i]));
            }
        }

        Self {
            rows: self.columns,
            columns: self.rows,
            data,
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + Default + Add<Output = K>,
{
    pub fn trace(&self) -> K {
        let mut acc = K::default();
        if !self.is_square() {
            return acc;
        }

        for i in 0..self.rows {
            acc = acc + self[i][i]
        }

        acc
    }
}

impl<K> Matrix<K>
where
    K: Copy
        + Mul<Output = K>
        + Default
        + PartialEq
        + Div<Output = K>
        + From<u8>
        + Neg<Output = K>
        + Add<Output = K>,
{
    pub fn row_echelon(&self) -> Self {
        let mut resp: Vec<Vector<K>> = self
            .data
            .iter()
            .map(|row| Vector::from(row.as_slice()))
            .collect();

        for row in 0..self.rows {
            let pivot_index = match Matrix::find_pivot_index(row, resp[row].data()) {
                Option::Some(v) => v,
                Option::None => break,
            };
            let pivot = resp[row][pivot_index];
            resp[row].scl(K::from(1u8) / pivot);

            for i in 0..self.rows {
                if i == row {
                    continue;
                }
                let row_pivot = resp[i][pivot_index];
                if row_pivot != K::from(0u8) {
                    let add = resp[row].scl_new(-row_pivot);
                    resp[i].add_inline(&add);
                }
            }
        }

        Self {
            columns: self.columns,
            rows: self.rows,
            data: resp.iter().map(|item| item.to_vec()).collect(),
        }
    }

    fn find_pivot_index(initial: usize, row: &Vec<K>) -> Option<usize> {
        for (index, &item) in row.iter().enumerate().skip(initial) {
            if item != K::from(0u8) {
                return Option::Some(index);
            }
        }

        Option::None
    }
}
