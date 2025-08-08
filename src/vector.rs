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

    pub fn iter(&self) -> Iter<'_, K> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K> {
        self.data.iter_mut()
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K>
where
    K: Clone,
{
    fn from(arr: [K; N]) -> Self {
        Self { data: arr.to_vec() }
    }
}

impl<K> Vector<K>
where
    K: Clone + Default,
{
    pub fn with_default(len: usize) -> Self {
        Self {
            data: vec![K::default(); len],
        }
    }
}

impl<K> Vector<K>
where
    K: Copy + Add<Output = K>,
{
    pub fn add(&mut self, other: &Self) {
        for (a, b) in self.data.iter_mut().zip(&other.data) {
            *a = *a + *b;
        }
    }

    pub fn add_new(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .zip(&other.data)
                .map(|(&a, &b)| a + b)
                .collect(),
        }
    }
}

impl<K> Vector<K>
where
    K: Copy + Sub<Output = K>,
{
    pub fn sub(&mut self, other: &Self) {
        for (a, b) in self.data.iter_mut().zip(&other.data) {
            *a = *a - *b;
        }
    }

    pub fn sub_new(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .zip(&other.data)
                .map(|(&a, &b)| a - b)
                .collect(),
        }
    }
}

impl<K> Vector<K>
where
    K: Copy + Mul<Output = K>,
{
    pub fn scl(&mut self, scalar: K) {
        for x in self.data.iter_mut() {
            *x = *x * scalar;
        }
    }

    pub fn scl_new(&self, scalar: K) -> Self {
        Self {
            data: self.data.iter().map(|&x| x * scalar).collect(),
        }
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
        self.iter()
            .zip(v.iter())
            .fold(K::default(), |acc, (&a, &b)| acc + (a * b))
    }
}

impl<K> Vector<K>
where
    K: Copy + Into<f32>,
{
    pub fn norm_1(&self) -> f32 {
        self.iter().fold(0.0, |acc, &x| acc + x.into().abs())
    }

    pub fn norm(&self) -> f32 {
        self.iter()
            .fold(0.0, |acc, &x| acc + x.into().powi(2))
            .sqrt()
    }

    pub fn norm_inf(&self) -> f32 {
        self.iter()
            .map(|&x| x.into().abs())
            .fold(f32::NEG_INFINITY, f32::max)
    }
}
