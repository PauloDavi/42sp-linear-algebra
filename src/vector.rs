use core::slice::{Iter, IterMut};
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<K> {
    data: Vec<K>,
}

impl<K> Vector<K> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, K> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K> {
        self.data.iter_mut()
    }

    pub fn as_slice(&self) -> &[K] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [K] {
        &mut self.data
    }

    pub fn into_inner(self) -> Vec<K> {
        self.data
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(arr: [K; N]) -> Self {
        Self { data: arr.into() }
    }
}

impl<K> From<&[K]> for Vector<K>
where
    K: Copy,
{
    fn from(data: &[K]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }
}

impl<K> From<Vec<K>> for Vector<K> {
    fn from(data: Vec<K>) -> Self {
        Self { data }
    }
}

impl<K> Vector<K>
where
    K: Default + Clone,
{
    pub fn zeros(len: usize) -> Self {
        Self {
            data: vec![K::default(); len],
        }
    }
}

impl<K> Vector<K>
where
    K: Copy + Add<Output = K>,
{
    pub fn add_inline(&mut self, other: &Self) {
        debug_assert_eq!(self.len(), other.len(), "Vector dimensions must match");

        for (a, &b) in self.data.iter_mut().zip(&other.data) {
            *a = *a + b;
        }
    }

    pub fn add_new(&self, other: &Self) -> Self {
        debug_assert_eq!(self.len(), other.len(), "Vector dimensions must match");

        let mut data = Vec::with_capacity(self.len());
        for (&a, &b) in self.data.iter().zip(&other.data) {
            data.push(a + b);
        }
        Self { data }
    }
}

impl<K> Vector<K>
where
    K: Copy + Sub<Output = K>,
{
    pub fn sub(&mut self, other: &Self) {
        debug_assert_eq!(self.len(), other.len(), "Vector dimensions must match");

        for (a, &b) in self.data.iter_mut().zip(&other.data) {
            *a = *a - b;
        }
    }

    pub fn sub_new(&self, other: &Self) -> Self {
        debug_assert_eq!(self.len(), other.len(), "Vector dimensions must match");

        let mut data = Vec::with_capacity(self.len());
        for (&a, &b) in self.data.iter().zip(&other.data) {
            data.push(a - b);
        }
        Self { data }
    }
}

impl<K> Vector<K>
where
    K: Copy + Mul<Output = K>,
{
    pub fn scl(&mut self, scalar: K) {
        for x in &mut self.data {
            *x = *x * scalar;
        }
    }

    pub fn scl_new(&self, scalar: K) -> Self {
        let mut data = Vec::with_capacity(self.len());
        for &x in &self.data {
            data.push(x * scalar);
        }
        Self { data }
    }
}

impl<K> Add for Vector<K>
where
    K: Copy + Add<Output = K>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self.add_new(&other)
    }
}

impl<K> Sub for Vector<K>
where
    K: Copy + Sub<Output = K>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self.sub_new(&other)
    }
}

impl<K> Mul<K> for Vector<K>
where
    K: Copy + Mul<Output = K>,
{
    type Output = Self;
    fn mul(self, scalar: K) -> Self::Output {
        self.scl_new(scalar)
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

impl<K> Vector<K>
where
    K: Copy + Default + Add<Output = K> + Mul<Output = K>,
{
    pub fn dot(&self, v: &Self) -> K {
        debug_assert_eq!(
            self.len(),
            v.len(),
            "Vector dimensions must match for dot product"
        );

        let mut acc = K::default();
        for (&a, &b) in self.data.iter().zip(&v.data) {
            acc = acc + (a * b);
        }
        acc
    }
}

impl<K> Vector<K>
where
    K: Copy + Into<f32>,
{
    pub fn norm_1(&self) -> f32 {
        let mut sum = 0.0f32;
        for &x in &self.data {
            sum += x.into().abs();
        }
        sum
    }

    pub fn norm(&self) -> f32 {
        let mut sum = 0.0f32;
        for &x in &self.data {
            let val = x.into();
            sum += val * val;
        }
        sum.sqrt()
    }

    pub fn norm_inf(&self) -> f32 {
        let mut max_val = 0.0f32;
        for &x in &self.data {
            let abs_val = x.into().abs();
            if abs_val > max_val {
                max_val = abs_val;
            }
        }
        max_val
    }
}

impl<K> Vector<K>
where
    K: Copy,
{
    pub fn to_vec(&self) -> Vec<K> {
        self.data.clone()
    }
}
