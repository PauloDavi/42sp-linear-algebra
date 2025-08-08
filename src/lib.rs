pub mod display;
pub mod errors;
pub mod interpolate;
pub mod linear_combination;
pub mod matrix;
pub mod vector;

pub use errors::{InterpolationError, LinearCombinationError};
pub use interpolate::lerp;
pub use linear_combination::linear_combination;
pub use matrix::Matrix;
pub use vector::Vector;
