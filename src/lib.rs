//! # Linear Algebra Library
//!
//! Uma biblioteca Rust para operações de álgebra linear, incluindo vetores e matrizes.
//!
//! ## Exemplos
//!
//! ### Trabalhando com Vetores
//!
//! ```rust
//! use linear_algebra::Vector;
//!
//! let v1 = Vector::new(&[1, 2, 3]);
//! let v2 = Vector::new(&[4, 5, 6]);
//!
//! // Adição de vetores
//! let mut result = v1.clone();
//! result.add(&v2).unwrap();
//! ```
//!
//! ### Combinação Linear
//!
//! ```rust
//! use linear_algebra::{Vector, linear_combination};
//!
//! let e1 = Vector::new(&[1.0, 0.0, 0.0]);
//! let e2 = Vector::new(&[0.0, 1.0, 0.0]);
//! let e3 = Vector::new(&[0.0, 0.0, 1.0]);
//!
//! let coefficients = [2.0, -1.0, 0.5];
//! let result = linear_combination(&[e1, e2, e3], &coefficients).unwrap();
//! ```

pub mod display;
pub mod errors;
pub mod linear_combination;
pub mod matrix;
pub mod types;
pub mod vector;

pub use linear_combination::linear_combination;
pub use matrix::Matrix;
pub use vector::Vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let v = Vector::new(&[1, 2, 3]);
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_vector_addition() {
        let mut v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5, 6]);

        v1.add(&v2).unwrap();
        assert_eq!(v1, Vector::new(&[5, 7, 9]));
    }

    #[test]
    fn test_linear_combination() {
        let e1 = Vector::new(&[1.0, 0.0]);
        let e2 = Vector::new(&[0.0, 1.0]);

        let coefficients = [2.0, 3.0];
        let result = linear_combination(&[e1, e2], &coefficients).unwrap();

        assert_eq!(result, Vector::new(&[2.0, 3.0]));
    }
}
