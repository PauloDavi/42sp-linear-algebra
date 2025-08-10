pub mod angle_cos;
pub mod cross_product;
pub mod display;
pub mod errors;
pub mod interpolate;
pub mod linear_combination;
pub mod matrix;
pub mod traits;
pub mod vector;

pub use angle_cos::angle_cos;
pub use cross_product::cross_product;
pub use errors::{InterpolationError, LinearCombinationError, MatrixInverseError};
pub use interpolate::lerp;
pub use linear_combination::linear_combination;
pub use matrix::Matrix;
pub use traits::{Magnitude, Negative, One, Zero};
pub use vector::Vector;
