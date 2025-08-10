use core::slice::{Iter, IterMut};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::{
    Vector,
    errors::MatrixInverseError,
    traits::{Conjugate, Magnitude, Negative, One, Zero},
};

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

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
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

    pub fn as_slice(&self) -> &[Vec<K>] {
        &self.data
    }

    pub fn into_inner(self) -> Vec<Vec<K>> {
        self.data
    }
}

impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(arr: [[K; C]; R]) -> Self {
        Self {
            rows: R,
            columns: C,
            data: arr.into_iter().map(|row| row.into()).collect(),
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy,
{
    pub fn transpose(&self) -> Self {
        let mut transposed_data = Vec::with_capacity(self.columns);
        for _ in 0..self.columns {
            transposed_data.push(Vec::with_capacity(self.rows));
        }

        for row in &self.data {
            for (j, &val) in row.iter().enumerate() {
                transposed_data[j].push(val);
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
    K: Copy + Conjugate,
{
    pub fn conjugate_transpose(&self) -> Self {
        let mut transposed_data = Vec::with_capacity(self.columns);
        for _ in 0..self.columns {
            transposed_data.push(Vec::with_capacity(self.rows));
        }

        for row in &self.data {
            for (j, &val) in row.iter().enumerate() {
                transposed_data[j].push(val.conjugate());
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
    K: Zero + Clone,
{
    pub fn zeros(rows: usize, columns: usize) -> Self {
        Self {
            columns,
            rows,
            data: vec![vec![K::zero(); columns]; rows],
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + Add<Output = K>,
{
    pub fn add(&mut self, other: &Self) {
        debug_assert_eq!(self.shape(), other.shape(), "Matrix dimensions must match");

        for (row_a, row_b) in self.data.iter_mut().zip(&other.data) {
            for (column_a, &column_b) in row_a.iter_mut().zip(row_b) {
                *column_a = *column_a + column_b;
            }
        }
    }

    pub fn add_new(&self, other: &Self) -> Self {
        debug_assert_eq!(self.shape(), other.shape(), "Matrix dimensions must match");

        let mut data = Vec::with_capacity(self.rows);
        for (row_a, row_b) in self.data.iter().zip(&other.data) {
            let mut new_row = Vec::with_capacity(self.columns);
            for (&column_a, &column_b) in row_a.iter().zip(row_b) {
                new_row.push(column_a + column_b);
            }
            data.push(new_row);
        }

        Self {
            columns: self.columns,
            rows: self.rows,
            data,
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + Sub<Output = K>,
{
    pub fn sub(&mut self, other: &Self) {
        debug_assert_eq!(self.shape(), other.shape(), "Matrix dimensions must match");

        for (row_a, row_b) in self.data.iter_mut().zip(&other.data) {
            for (column_a, &column_b) in row_a.iter_mut().zip(row_b) {
                *column_a = *column_a - column_b;
            }
        }
    }

    pub fn sub_new(&self, other: &Self) -> Self {
        debug_assert_eq!(self.shape(), other.shape(), "Matrix dimensions must match");

        let mut data = Vec::with_capacity(self.rows);
        for (row_a, row_b) in self.data.iter().zip(&other.data) {
            let mut new_row = Vec::with_capacity(self.columns);
            for (&column_a, &column_b) in row_a.iter().zip(row_b) {
                new_row.push(column_a - column_b);
            }
            data.push(new_row);
        }

        Self {
            columns: self.columns,
            rows: self.rows,
            data,
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + Mul<Output = K>,
{
    pub fn scl(&mut self, scalar: K) {
        for row in &mut self.data {
            for column in row {
                *column = *column * scalar;
            }
        }
    }

    pub fn scl_new(&self, scalar: K) -> Self {
        let mut data = Vec::with_capacity(self.rows);
        for row in &self.data {
            let mut new_row = Vec::with_capacity(self.columns);
            for &column in row {
                new_row.push(column * scalar);
            }
            data.push(new_row);
        }

        Self {
            columns: self.columns,
            rows: self.rows,
            data,
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
    K: Copy + Zero + Add<Output = K> + Mul<Output = K>,
{
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        debug_assert_eq!(self.rows, vec.len(), "Matrix rows must match vector length");

        let mut result = Vec::with_capacity(self.rows);

        for column in 0..self.columns {
            let mut sum = K::zero();
            for (row, &vec_val) in vec.iter().enumerate() {
                sum = sum + (self.data[row][column] * vec_val)
            }
            result.push(sum);
        }

        Vector::from(result)
    }

    pub fn mul_mat(&self, other: &Self) -> Self {
        debug_assert_eq!(
            self.columns, other.rows,
            "Matrix dimensions must be compatible for multiplication"
        );

        let mut result_data = Vec::with_capacity(self.rows);

        for i in 0..self.rows {
            let mut row = Vec::with_capacity(other.columns);
            for j in 0..other.columns {
                let mut sum = K::zero();
                for k in 0..self.columns {
                    sum = sum + (self.data[i][k] * other.data[k][j]);
                }
                row.push(sum);
            }
            result_data.push(row);
        }

        Self {
            rows: self.rows,
            columns: other.columns,
            data: result_data,
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + Zero + Add<Output = K>,
{
    pub fn trace(&self) -> K {
        if !self.is_square() {
            return K::zero();
        }

        let mut acc = K::zero();
        for i in 0..self.rows {
            acc = acc + self[i][i];
        }
        acc
    }
}

impl<K> Matrix<K>
where
    K: Copy
        + Mul<Output = K>
        + Zero
        + One
        + PartialEq
        + Div<Output = K>
        + Neg<Output = K>
        + Add<Output = K>,
{
    pub fn row_echelon(&self) -> Self {
        let mut rows: Vec<Vector<K>> = self
            .data
            .iter()
            .map(|row| Vector::from(row.as_slice()))
            .collect();

        for row in 0..self.rows {
            match rows[row]
                .as_slice()
                .iter()
                .enumerate()
                .skip(row)
                .find_map(|(i, &val)| if val != K::zero() { Some(i) } else { None })
            {
                Some(pivot_idx) => {
                    let pivot = rows[row][pivot_idx];
                    rows[row].scl(K::one() / pivot);

                    for i in 0..self.rows {
                        let factor = rows[i][pivot_idx];
                        if i == row || factor == K::zero() {
                            continue;
                        }

                        let scaled = rows[row].scl_new(-factor);
                        rows[i].add_inline(&scaled);
                    }
                }
                None => break,
            };
        }

        Self {
            columns: self.columns,
            rows: self.rows,
            data: rows.into_iter().map(|v| v.to_vec()).collect(),
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy,
{
    pub fn sub_matrix(&self, remove_row: usize, remove_col: usize) -> Self {
        let mut data = Vec::with_capacity(self.rows - 1);

        for (i, row) in self.data.iter().enumerate() {
            if i == remove_row {
                continue;
            }

            let mut new_row = Vec::with_capacity(self.columns - 1);
            for (j, &val) in row.iter().enumerate() {
                if j == remove_col {
                    continue;
                }
                new_row.push(val);
            }
            data.push(new_row);
        }

        Self {
            rows: self.rows - 1,
            columns: self.columns - 1,
            data,
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Zero + One + Negative,
{
    pub fn determinant(&self) -> K {
        if !self.is_square() {
            return K::zero();
        }

        match self.rows {
            1 => self[0][0],
            2 => (self[0][0] * self[1][1]) - (self[0][1] * self[1][0]),
            3 => {
                let a = self[0][0] * self[1][1] * self[2][2];
                let b = self[0][1] * self[1][2] * self[2][0];
                let c = self[0][2] * self[1][0] * self[2][1];
                let d = self[0][2] * self[1][1] * self[2][0];
                let e = self[0][1] * self[1][0] * self[2][2];
                let f = self[0][0] * self[1][2] * self[2][1];

                (a + b + c) - (d + e + f)
            }
            4 => {
                let mut det = K::zero();
                for col in 0..4 {
                    let minor = self.sub_matrix(0, col);
                    let sign = if col % 2 == 0 {
                        K::one()
                    } else {
                        K::negative_one()
                    };
                    det = det + sign * self[0][col] * minor.determinant();
                }
                det
            }
            _ => K::zero(),
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy
        + Zero
        + One
        + Add<Output = K>
        + Sub<Output = K>
        + Div<Output = K>
        + Mul<Output = K>
        + PartialEq,
{
    pub fn inverse(&self) -> Result<Self, MatrixInverseError> {
        if !self.is_square() {
            return Err(MatrixInverseError::NotSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }

        let n = self.rows;
        let mut a = self.data.clone();
        let mut inv = vec![vec![K::zero(); n]; n];
        for (i, row) in inv.iter_mut().enumerate().take(n) {
            row[i] = K::one();
        }

        for i in 0..n {
            let pivot = a[i][i];
            if pivot == K::zero() {
                return Err(MatrixInverseError::Singular);
            }

            for j in 0..n {
                a[i][j] = a[i][j] / pivot;
                inv[i][j] = inv[i][j] / pivot;
            }

            for k in 0..n {
                if k != i {
                    let factor = a[k][i];
                    for j in 0..n {
                        a[k][j] = a[k][j] - factor * a[i][j];
                        inv[k][j] = inv[k][j] - factor * inv[i][j];
                    }
                }
            }
        }

        Ok(Self {
            rows: n,
            columns: n,
            data: inv,
        })
    }
}

impl<K> Matrix<K>
where
    K: Copy
        + PartialOrd
        + Sub<Output = K>
        + Div<Output = K>
        + Mul<Output = K>
        + Magnitude<Output = f32>,
{
    pub fn rank(&mut self) -> usize {
        let mut rank = 0;
        let mut row = 0;

        for col in 0..self.columns {
            let mut pivot_row = row;
            while pivot_row < self.rows && (self.data[pivot_row][col].magnitude() < 1e-10) {
                pivot_row += 1;
            }

            if pivot_row < self.rows {
                if pivot_row != row {
                    self.data.swap(pivot_row, row);
                }

                let pivot_val = self.data[row][col];
                for j in col..self.columns {
                    self.data[row][j] = self.data[row][j] / pivot_val;
                }

                for i in (row + 1)..self.rows {
                    let factor = self.data[i][col];
                    for j in col..self.columns {
                        self.data[i][j] = self.data[i][j] - factor * self.data[row][j];
                    }
                }

                rank += 1;
                row += 1;
            }
        }

        rank
    }
}
