//! # Library Error Types
//!
//! This module defines all specific error types that can occur
//! during linear algebra operations.

use std::error::Error;

/// Errors related to linear combination operations.
///
/// This error type occurs when there are incompatibilities in vector
/// dimensions during linear combination operations.
#[derive(Debug)]
pub enum LinearCombinationError {
    /// Error when vectors have incompatible dimensions.
    ///
    /// Indicates that the vectors involved in the operation do not have
    /// the same number of elements, making linear combination impossible.
    VectorsDimensionMismatch {
        /// Expected number of elements
        expected_len: usize,
        /// Found number of elements
        founded_len: usize,
    },
}

impl Error for LinearCombinationError {}

/// Errors related to interpolation operations.
///
/// This error type occurs when invalid parameters are provided
/// to interpolation functions.
#[derive(Debug)]
pub enum InterpolationError {
    /// Error when the interpolation parameter is outside the valid range.
    ///
    /// The parameter `t` for linear interpolation must be between 0.0 and 1.0.
    InvalidParameterT {
        /// The invalid value of t provided
        t: f32,
    },
}

impl Error for InterpolationError {}

/// Errors related to matrix inversion.
///
/// This error type occurs when a matrix cannot be inverted
/// due to specific mathematical properties.
#[derive(Debug)]
pub enum MatrixInverseError {
    /// Error when attempting to invert a non-square matrix.
    ///
    /// Only square matrices (same number of rows and columns)
    /// can have a mathematical inverse.
    NotSquare {
        /// Number of rows in the matrix
        rows: usize,
        /// Number of columns in the matrix
        columns: usize,
    },
    /// Error when the matrix is singular (zero determinant).
    ///
    /// Singular matrices do not have a mathematical inverse.
    Singular,
}

impl Error for MatrixInverseError {}
