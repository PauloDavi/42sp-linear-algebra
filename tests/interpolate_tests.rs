use linear_algebra::{InterpolationError, Matrix, Vector, lerp};

#[cfg(test)]
mod interpolate_tests {
    use super::*;

    // Vector interpolation tests
    #[test]
    fn test_lerp_vector_integers() {
        let u = Vector::from([0.0, 0.0, 0.0]);
        let v = Vector::from([10.0, 20.0, 30.0]);

        let result = lerp(u, v, 0.5).unwrap();
        assert_eq!(result, Vector::from([5.0, 10.0, 15.0]));
    }

    #[test]
    fn test_lerp_vector_floats() {
        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([5.0, 6.0, 7.0]);

        let result = lerp(u, v, 0.25).unwrap();
        assert_eq!(result, Vector::from([2.0, 3.0, 4.0]));
    }

    #[test]
    fn test_lerp_vector_t_zero() {
        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([10.0, 20.0, 30.0]);

        let result = lerp(u, v, 0.0).unwrap();
        assert_eq!(result, Vector::from([1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_lerp_vector_t_one() {
        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([10.0, 20.0, 30.0]);

        let result = lerp(u, v, 1.0).unwrap();
        assert_eq!(result, Vector::from([10.0, 20.0, 30.0]));
    }

    #[test]
    fn test_lerp_vector_negative_values() {
        let u = Vector::from([-5.0, -10.0, -15.0]);
        let v = Vector::from([5.0, 10.0, 15.0]);

        let result = lerp(u, v, 0.5).unwrap();
        assert_eq!(result, Vector::from([0.0, 0.0, 0.0]));
    }

    #[test]
    fn test_lerp_vector_single_element() {
        let u = Vector::from([0.0]);
        let v = Vector::from([100.0]);

        let result = lerp(u, v, 0.2).unwrap();
        assert_eq!(result, Vector::from([20.0]));
    }

    // Matrix interpolation tests
    #[test]
    fn test_lerp_matrix_2x2() {
        let u = Matrix::from([[0.0, 0.0], [0.0, 0.0]]);
        let v = Matrix::from([[10.0, 20.0], [30.0, 40.0]]);

        let result = lerp(u, v, 0.5).unwrap();
        assert_eq!(result[0][0], 5.0);
        assert_eq!(result[0][1], 10.0);
        assert_eq!(result[1][0], 15.0);
        assert_eq!(result[1][1], 20.0);
    }

    #[test]
    fn test_lerp_matrix_3x3() {
        let u = Matrix::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let v = Matrix::from([[10.0, 20.0, 30.0], [40.0, 50.0, 60.0], [70.0, 80.0, 90.0]]);

        let result = lerp(u, v, 0.25).unwrap();
        assert_eq!(result[0][0], 3.25);
        assert_eq!(result[1][1], 16.25);
        assert_eq!(result[2][2], 29.25);
    }

    #[test]
    fn test_lerp_matrix_floats() {
        let u = Matrix::from([[1.5, 2.5], [3.5, 4.5]]);
        let v = Matrix::from([[5.5, 6.5], [7.5, 8.5]]);

        let result = lerp(u, v, 0.5).unwrap();
        assert_eq!(result[0][0], 3.5);
        assert_eq!(result[0][1], 4.5);
        assert_eq!(result[1][0], 5.5);
        assert_eq!(result[1][1], 6.5);
    }

    #[test]
    fn test_lerp_matrix_t_zero() {
        let u = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let v = Matrix::from([[10.0, 20.0], [30.0, 40.0]]);

        let result = lerp(u, v, 0.0).unwrap();
        assert_eq!(result[0][0], 1.0);
        assert_eq!(result[0][1], 2.0);
        assert_eq!(result[1][0], 3.0);
        assert_eq!(result[1][1], 4.0);
    }

    #[test]
    fn test_lerp_matrix_t_one() {
        let u = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let v = Matrix::from([[10.0, 20.0], [30.0, 40.0]]);

        let result = lerp(u, v, 1.0).unwrap();
        assert_eq!(result[0][0], 10.0);
        assert_eq!(result[0][1], 20.0);
        assert_eq!(result[1][0], 30.0);
        assert_eq!(result[1][1], 40.0);
    }

    #[test]
    fn test_lerp_matrix_single_element() {
        let u = Matrix::from([[5.0]]);
        let v = Matrix::from([[15.0]]);

        let result = lerp(u, v, 0.2).unwrap();
        assert_eq!(result[0][0], 7.0);
    }

    #[test]
    fn test_lerp_matrix_rectangular() {
        let u = Matrix::from([[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]]);
        let v = Matrix::from([[10.0, 20.0, 30.0, 40.0], [50.0, 60.0, 70.0, 80.0]]);

        let result = lerp(u, v, 0.1).unwrap();
        assert_eq!(result[0][0], 1.9);
        assert_eq!(result[0][3], 7.6);
        assert_eq!(result[1][0], 9.5);
        assert_eq!(result[1][3], 15.2);
    }

    // Scalar interpolation tests
    #[test]
    fn test_lerp_scalar_integers() {
        let result = lerp(0.0, 10.0, 0.3).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_lerp_scalar_floats() {
        let result = lerp(1.0, 5.0, 0.25).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_lerp_scalar_negative() {
        let result = lerp(-10.0, 10.0, 0.5).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_lerp_scalar_same_values() {
        let result = lerp(5.0, 5.0, 0.7).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_lerp_scalar_large_numbers() {
        let result = lerp(1000.0, 2000.0, 0.4).unwrap();
        assert_eq!(result, 1400.0);
    }

    #[test]
    fn test_lerp_scalar_f32() {
        let result = lerp(1.5f32, 3.5f32, 0.5).unwrap();
        assert_eq!(result, 2.5);
    }

    // Error cases
    #[test]
    fn test_lerp_vector_invalid_t_negative() {
        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([4.0, 5.0, 6.0]);

        let result = lerp(u, v, -0.1);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InterpolationError::InvalidParameterT { t: -0.1 }
        ));
    }

    #[test]
    fn test_lerp_vector_invalid_t_greater_than_one() {
        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([4.0, 5.0, 6.0]);

        let result = lerp(u, v, 1.5);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InterpolationError::InvalidParameterT { t: 1.5 }
        ));
    }

    #[test]
    fn test_lerp_matrix_invalid_t_negative() {
        let u = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let v = Matrix::from([[5.0, 6.0], [7.0, 8.0]]);

        let result = lerp(u, v, -0.5);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InterpolationError::InvalidParameterT { t: -0.5 }
        ));
    }

    #[test]
    fn test_lerp_scalar_invalid_t_greater_than_one() {
        let result = lerp(0.0, 10.0, 2.0);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InterpolationError::InvalidParameterT { t: 2.0 }
        ));
    }

    // Edge cases with very small and large t values
    #[test]
    fn test_lerp_vector_t_very_small() {
        let u = Vector::from([100.0, 200.0, 300.0]);
        let v = Vector::from([0.0, 0.0, 0.0]);

        let result = lerp(u, v, 0.001).unwrap();
        assert_eq!(result[0], 99.9);
        assert_eq!(result[1], 199.8);
        assert_eq!(result[2], 299.7);
    }

    #[test]
    fn test_lerp_vector_t_very_close_to_one() {
        let u = Vector::from([0.0, 0.0, 0.0]);
        let v = Vector::from([100.0, 200.0, 300.0]);

        let result = lerp(u, v, 0.999).unwrap();
        assert_eq!(result[0], 99.9);
        assert_eq!(result[1], 199.8);
        assert_eq!(result[2], 299.7);
    }

    #[test]
    fn test_lerp_matrix_with_mixed_positive_negative() {
        let u = Matrix::from([[-5.0, 10.0], [15.0, -20.0]]);
        let v = Matrix::from([[5.0, -10.0], [-15.0, 20.0]]);

        let result = lerp(u, v, 0.5).unwrap();
        assert_eq!(result[0][0], 0.0);
        assert_eq!(result[0][1], 0.0);
        assert_eq!(result[1][0], 0.0);
        assert_eq!(result[1][1], 0.0);
    }

    #[test]
    fn test_lerp_vector_fractional_t() {
        let u = Vector::from([10.0, 20.0, 30.0]);
        let v = Vector::from([40.0, 50.0, 60.0]);

        let result = lerp(u, v, 1.0 / 3.0).unwrap();
        assert!((result[0] - 20.0).abs() < 1e-6);
        assert!((result[1] - 30.0).abs() < 1e-6);
        assert!((result[2] - 40.0).abs() < 1e-6);
    }

    #[test]
    fn test_lerp_matrix_with_zeros() {
        let u = Matrix::from([[0.0, 0.0], [0.0, 0.0]]);
        let v = Matrix::from([[0.0, 0.0], [0.0, 0.0]]);

        let result = lerp(u, v, 0.7).unwrap();
        assert_eq!(result[0][0], 0.0);
        assert_eq!(result[0][1], 0.0);
        assert_eq!(result[1][0], 0.0);
        assert_eq!(result[1][1], 0.0);
    }
}
