use linear_algebra::{Vector, linear_combination};

#[cfg(test)]
mod linear_combination_tests {
    use super::*;

    #[test]
    fn test_linear_combination_basic_2d() {
        let e1 = Vector::new(&[1.0, 0.0]);
        let e2 = Vector::new(&[0.0, 1.0]);
        let coefficients = [2.0, 3.0];

        let result = linear_combination(&[e1, e2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[2.0, 3.0]));
    }

    #[test]
    fn test_linear_combination_basic_3d() {
        let e1 = Vector::new(&[1.0, 0.0, 0.0]);
        let e2 = Vector::new(&[0.0, 1.0, 0.0]);
        let e3 = Vector::new(&[0.0, 0.0, 1.0]);
        let coefficients = [2.0, -1.0, 0.5];

        let result = linear_combination(&[e1, e2, e3], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[2.0, -1.0, 0.5]));
    }

    #[test]
    fn test_linear_combination_integers() {
        let v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5, 6]);
        let coefficients = [2, 3];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[14, 19, 24]));
    }

    #[test]
    fn test_linear_combination_single_vector() {
        let v = Vector::new(&[1, 2, 3]);
        let coefficients = [5];

        let result = linear_combination(&[v], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[5, 10, 15]));
    }

    #[test]
    fn test_linear_combination_zero_coefficient() {
        let v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5, 6]);
        let coefficients = [0, 1];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[4, 5, 6]));
    }

    #[test]
    fn test_linear_combination_negative_coefficients() {
        let v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5, 6]);
        let coefficients = [-1, -2];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[-9, -12, -15]));
    }

    #[test]
    fn test_linear_combination_mixed_coefficients() {
        let v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5, 6]);
        let v3 = Vector::new(&[7, 8, 9]);
        let coefficients = [1, -1, 2];

        let result = linear_combination(&[v1, v2, v3], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[11, 13, 15]));
    }

    #[test]
    fn test_linear_combination_empty_vectors() {
        let vectors: Vec<Vector<f64>> = vec![];
        let coefficients: Vec<f64> = vec![];

        let result = linear_combination(&vectors, &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[]));
    }

    #[test]
    fn test_linear_combination_zero_length_vectors() {
        let v1: Vector<i32> = Vector::new(&[]);
        let v2: Vector<i32> = Vector::new(&[]);
        let coefficients = [1, 2];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[]));
    }

    // Testes de erro - dimensões incompatíveis
    #[test]
    fn test_linear_combination_dimension_mismatch_vectors_coefficients() {
        let v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5, 6]);
        let coefficients = [1];

        let result = linear_combination(&[v1, v2], &coefficients);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            linear_algebra::errors::VectorError::LinearCombinationDimensionMismatch
        ));
    }

    #[test]
    fn test_linear_combination_dimension_mismatch_more_coefficients() {
        let v1 = Vector::new(&[1, 2, 3]);
        let coefficients = [1, 2];

        let result = linear_combination(&[v1], &coefficients);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            linear_algebra::errors::VectorError::LinearCombinationDimensionMismatch
        ));
    }

    #[test]
    fn test_linear_combination_vector_dimension_mismatch() {
        let v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5]);
        let coefficients = [1, 2];

        let result = linear_combination(&[v1, v2], &coefficients);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            linear_algebra::errors::VectorError::DimensionMismatch
        ));
    }

    #[test]
    fn test_linear_combination_multiple_vector_dimension_mismatch() {
        let v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5, 6]);
        let v3 = Vector::new(&[7, 8]);
        let coefficients = [1, 2, 3];

        let result = linear_combination(&[v1, v2, v3], &coefficients);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            linear_algebra::errors::VectorError::DimensionMismatch
        ));
    }

    #[test]
    fn test_linear_combination_i8() {
        let v1 = Vector::new(&[1i8, 2i8, 3i8]);
        let v2 = Vector::new(&[4i8, 5i8, 6i8]);
        let coefficients = [2i8, 1i8];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[6i8, 9i8, 12i8]));
    }

    #[test]
    fn test_linear_combination_i64() {
        let v1 = Vector::new(&[1000000i64, 2000000i64, 3000000i64]);
        let v2 = Vector::new(&[4000000i64, 5000000i64, 6000000i64]);
        let coefficients = [2i64, 1i64];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[6000000i64, 9000000i64, 12000000i64]));
    }

    #[test]
    fn test_linear_combination_f32() {
        let v1 = Vector::new(&[1.5f32, 2.5f32, 3.5f32]);
        let v2 = Vector::new(&[0.5f32, 1.5f32, 2.5f32]);
        let coefficients = [2.0f32, 4.0f32];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[5.0f32, 11.0f32, 17.0f32]));
    }

    // Testes com valores especiais de float
    #[test]
    fn test_linear_combination_with_zero_float() {
        let v1 = Vector::new(&[1.0, 2.0, 3.0]);
        let v2 = Vector::new(&[4.0, 5.0, 6.0]);
        let coefficients = [0.0, 0.0];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[0.0, 0.0, 0.0]));
    }

    #[test]
    fn test_linear_combination_with_fractional_coefficients() {
        let v1 = Vector::new(&[2.0, 4.0, 6.0]);
        let v2 = Vector::new(&[1.0, 2.0, 3.0]);
        let coefficients = [0.5, 0.25];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[1.25, 2.5, 3.75]));
    }

    // Testes de casos extremos
    #[test]
    fn test_linear_combination_large_vectors() {
        let v1 = Vector::new(&[1; 1000]);
        let v2 = Vector::new(&[2; 1000]);
        let coefficients = [3, 4];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        let expected = Vector::new(&[11; 1000]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_linear_combination_many_vectors() {
        let vectors = vec![
            Vector::new(&[1, 0, 0]),
            Vector::new(&[0, 1, 0]),
            Vector::new(&[0, 0, 1]),
            Vector::new(&[1, 1, 1]),
            Vector::new(&[2, 3, 4]),
        ];
        let coefficients = [1, 2, 3, 4, 5];

        let result = linear_combination(&vectors, &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[15, 21, 27]));
    }

    // Testes de propriedades matemáticas
    #[test]
    fn test_linear_combination_associativity() {
        let v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5, 6]);
        let v3 = Vector::new(&[7, 8, 9]);

        let result1 = {
            let temp = linear_combination(&[v1.clone(), v2.clone()], &[2, 3]).unwrap();
            linear_combination(&[temp, v3.clone()], &[1, 4]).unwrap()
        };

        let result2 = {
            let temp = linear_combination(&[v2.clone(), v3.clone()], &[3, 4]).unwrap();
            linear_combination(&[v1.clone(), temp], &[2, 1]).unwrap()
        };

        let result3 = linear_combination(&[v1, v2, v3], &[2, 3, 4]).unwrap();

        assert_eq!(result1, result3);
        assert_eq!(result2, result3);
    }

    #[test]
    fn test_linear_combination_distributivity() {
        let v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5, 6]);
        let a = 2;
        let b = 3;
        let c = 5;

        let temp = linear_combination(&[v1.clone(), v2.clone()], &[a, b]).unwrap();
        let result1 = linear_combination(&[temp], &[c]).unwrap();

        let result2 = linear_combination(&[v1, v2], &[c * a, c * b]).unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_linear_combination_identity() {
        let v = Vector::new(&[1, 2, 3, 4, 5]);

        let result = linear_combination(&[v.clone()], &[1]).unwrap();
        assert_eq!(result, v);
    }

    #[test]
    fn test_linear_combination_zero() {
        let v1 = Vector::new(&[1, 2, 3]);
        let v2 = Vector::new(&[4, 5, 6]);

        let result = linear_combination(&[v1, v2], &[0, 0]).unwrap();
        assert_eq!(result, Vector::new(&[0, 0, 0]));
    }

    #[test]
    fn test_linear_combination_single_element_vectors() {
        let v1 = Vector::new(&[5]);
        let v2 = Vector::new(&[10]);
        let coefficients = [2, 3];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[40]));
    }

    #[test]
    fn test_linear_combination_very_small_coefficients() {
        let v1 = Vector::new(&[1000000.0, 2000000.0]);
        let v2 = Vector::new(&[3000000.0, 4000000.0]);
        let coefficients = [0.000001, 0.000002];

        let result = linear_combination(&[v1, v2], &coefficients).unwrap();
        assert_eq!(result, Vector::new(&[7.0, 10.0]));
    }
}
