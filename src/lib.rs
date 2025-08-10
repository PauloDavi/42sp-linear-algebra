//! # Linear Algebra Library
//!
//! A comprehensive linear algebra library in Rust, offering support for operations
//! with vectors, matrices, and complex numbers.
//!
//! ## Main Features
//!
//! - **Vectors**: Basic operations like addition, scalar multiplication, dot product, and norm
//! - **Matrices**: Matrix operations including multiplication, transposition, and inversion
//! - **Complex Numbers**: Complete support for complex arithmetic with conjugate operations
//! - **Interpolation**: Linear interpolation functions
//! - **Cross Products**: 3D cross product calculations
//! - **Linear Combinations**: Linear combination operations for vectors
//!
//! ## Usage Examples
//!
//! ### Vector Operations
//!
//! ```rust
//! use linear_algebra::Vector;
//!
//! let v1 = Vector::from(vec![1.0, 2.0, 3.0]);
//! let v2 = Vector::from(vec![4.0, 5.0, 6.0]);
//!
//! // Vector addition
//! let sum = v1.add_new(&v2);
//!
//! // Dot product
//! let dot = v1.dot(&v2);
//!
//! // Vector norm
//! let norm = v1.norm();
//! ```
//!
//! ### Matrix Operations
//!
//! ```rust
//! use linear_algebra::Matrix;
//!
//! let matrix = Matrix::from([
//!     [1.0, 2.0],
//!     [3.0, 4.0]
//! ]);
//!
//! // Transposition
//! let transposed = matrix.transpose();
//!
//! // Matrix multiplication
//! let result = matrix.mul_mat(&transposed);
//! ```
//!
//! ### Complex Numbers
//!
//! ```rust
//! use linear_algebra::{Complex, traits::Magnitude};
//!
//! let z1 = Complex::new(3.0, 4.0);
//! let z2 = Complex::new(1.0, 2.0);
//!
//! // Complex addition
//! let sum = z1 + z2;
//!
//! // Conjugate
//! let conj = z1.conjugate();
//!
//! // Magnitude
//! let mag = z1.magnitude();
//! ```

pub mod angle_cos;
pub mod complex;
pub mod cross_product;
pub mod display;
pub mod errors;
pub mod interpolate;
pub mod linear_combination;
pub mod matrix;
pub mod traits;
pub mod vector;

pub use angle_cos::angle_cos;
pub use complex::Complex;
pub use cross_product::cross_product;
pub use errors::{InterpolationError, LinearCombinationError, MatrixInverseError};
pub use interpolate::lerp;
pub use linear_combination::linear_combination;
pub use matrix::Matrix;
pub use traits::{Conjugate, Magnitude, Negative, One, Zero};
pub use vector::Vector;
