use std::error::Error;

#[derive(Debug)]
pub enum LinearCombinationError {
    VectorsDimensionMismatch {
        expected_len: usize,
        founded_len: usize,
    },
}

impl Error for LinearCombinationError {}

#[derive(Debug)]
pub enum InterpolationError {
    InvalidParameterT { t: f32 },
}

impl Error for InterpolationError {}

#[derive(Debug)]
pub enum MatrixInverseError {
    NotSquare { rows: usize, columns: usize },
    Singular,
}

impl Error for MatrixInverseError {}
