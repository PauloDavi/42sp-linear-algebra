use core::slice::{Iter, IterMut};
use std::ops::{Add, Index, IndexMut, Mul, Sub};

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
    type Output = Matrix<K>;
    fn add(self, other: Matrix<K>) -> Self::Output {
        self.add_new(&other)
    }
}

impl<K> Sub for Matrix<K>
where
    K: Copy + Sub<Output = K>,
{
    type Output = Matrix<K>;
    fn sub(self, other: Matrix<K>) -> Self::Output {
        self.sub_new(&other)
    }
}

impl<K> Mul<K> for Matrix<K>
where
    K: Copy + Mul<Output = K>,
{
    type Output = Matrix<K>;
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
