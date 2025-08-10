#[cfg(test)]
mod display_tests {
    use linear_algebra::{Complex, Matrix, Vector};

    #[test]
    fn test_vector_display_integers() {
        let v = Vector::from([1, 2, 3]);
        assert_eq!(format!("{v}"), "[1, 2, 3]");
    }

    #[test]
    fn test_vector_display_floats() {
        let v = Vector::from([1.0, 2.5, 3.15]);
        assert_eq!(format!("{v}"), "[1, 2.5, 3.15]");
    }

    #[test]
    fn test_vector_display_single_element() {
        let v = Vector::from([42]);
        assert_eq!(format!("{v}"), "[42]");
    }

    #[test]
    fn test_vector_display_empty() {
        let v: Vector<i32> = Vector::from([]);
        assert_eq!(format!("{v}"), "[]");
    }

    #[test]
    fn test_vector_display_negative_numbers() {
        let v = Vector::from([-1, -2, -3]);
        assert_eq!(format!("{v}"), "[-1, -2, -3]");
    }

    #[test]
    fn test_vector_display_mixed_numbers() {
        let v = Vector::from([-1, 0, 1, 2]);
        assert_eq!(format!("{v}"), "[-1, 0, 1, 2]");
    }

    #[test]
    fn test_vector_display_large_numbers() {
        let v = Vector::from([1000, 2000, 3000]);
        assert_eq!(format!("{v}"), "[1000, 2000, 3000]");
    }

    #[test]
    fn test_matrix_display_2x2() {
        let m = Matrix::from([[1, 2], [3, 4]]);
        let expected = "[1, 2]\n[3, 4]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_3x3() {
        let m = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let expected = "[1, 2, 3]\n[4, 5, 6]\n[7, 8, 9]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_1x1() {
        let m = Matrix::from([[42]]);
        assert_eq!(format!("{m}"), "[42]");
    }

    #[test]
    fn test_matrix_display_row_vector() {
        let m = Matrix::from([[1, 2, 3, 4, 5]]);
        assert_eq!(format!("{m}"), "[1, 2, 3, 4, 5]");
    }

    #[test]
    fn test_matrix_display_column_vector() {
        let m = Matrix::from([[1], [2], [3]]);
        let expected = "[1]\n[2]\n[3]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_with_negative_numbers() {
        let m = Matrix::from([[-1, 2], [3, -4]]);
        let expected = "[-1,  2]\n[ 3, -4]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_different_width_numbers() {
        let m = Matrix::from([[1, 10, 100], [2000, 5, 60]]);
        let expected = "[   1, 10, 100]\n[2000,  5,  60]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_float_numbers() {
        let m = Matrix::from([[1.0, 2.5], [3.15, 4.0]]);
        let expected = "[   1, 2.5]\n[3.15,   4]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_alignment_with_mixed_lengths() {
        let m = Matrix::from([[1, 1000], [99, 8]]);
        let expected = "[ 1, 1000]\n[99,    8]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_rectangular_4x2() {
        let m = Matrix::from([[1, 2], [3, 4], [5, 6], [7, 8]]);
        let expected = "[1, 2]\n[3, 4]\n[5, 6]\n[7, 8]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_rectangular_2x4() {
        let m = Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8]]);
        let expected = "[1, 2, 3, 4]\n[5, 6, 7, 8]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_zeros() {
        let m = Matrix::from([[0, 0], [0, 0]]);
        let expected = "[0, 0]\n[0, 0]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_large_matrix() {
        let m = Matrix::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        let expected = "[ 1,  2,  3,  4]\n[ 5,  6,  7,  8]\n[ 9, 10, 11, 12]\n[13, 14, 15, 16]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_vector_debug_format() {
        let v = Vector::from([1, 2, 3]);
        let debug_str = format!("{v:?}");
        assert!(debug_str.contains("Vector"));
        assert!(debug_str.contains("data"));
    }

    #[test]
    fn test_matrix_debug_format() {
        let m = Matrix::from([[1, 2], [3, 4]]);
        let debug_str = format!("{m:?}");
        assert!(debug_str.contains("Matrix"));
        assert!(debug_str.contains("rows"));
        assert!(debug_str.contains("columns"));
        assert!(debug_str.contains("data"));
    }

    #[test]
    fn test_vector_display_i8() {
        let v = Vector::from([1i8, 2i8, 3i8]);
        assert_eq!(format!("{v}"), "[1, 2, 3]");
    }

    #[test]
    fn test_vector_display_i16() {
        let v = Vector::from([1000i16, 2000i16, 3000i16]);
        assert_eq!(format!("{v}"), "[1000, 2000, 3000]");
    }

    #[test]
    fn test_vector_display_i64() {
        let v = Vector::from([1000000i64, 2000000i64, 3000000i64]);
        assert_eq!(format!("{v}"), "[1000000, 2000000, 3000000]");
    }

    #[test]
    fn test_matrix_display_i8() {
        let m = Matrix::from([[1i8, 2i8], [3i8, 4i8]]);
        let expected = "[1, 2]\n[3, 4]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_f64() {
        let m = Matrix::from([[1.123f64, 2.456f64], [3.789f64, 4.0f64]]);
        let expected = "[1.123, 2.456]\n[3.789,     4]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_vector_display_with_scientific_notation() {
        let v = Vector::from([1e6, 2e-3, 3.15159]);
        let formatted = format!("{v}");
        assert!(formatted.starts_with('['));
        assert!(formatted.ends_with(']'));
        assert!(formatted.contains(','));
    }

    #[test]
    fn test_matrix_display_very_different_column_widths() {
        let m = Matrix::from([[1, 12345], [67890, 2]]);
        let expected = "[    1, 12345]\n[67890,     2]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_matrix_display_three_columns_different_widths() {
        let m = Matrix::from([[1, 22, 333], [4444, 5, 66]]);
        let expected = "[   1, 22, 333]\n[4444,  5,  66]";
        assert_eq!(format!("{m}"), expected);
    }

    #[test]
    fn test_complex_display_zero() {
        let c = Complex::new(0.0, 0.0);
        assert_eq!(format!("{c}"), "0");
    }

    #[test]
    fn test_complex_display_real_only() {
        let c = Complex::new(5.0, 0.0);
        assert_eq!(format!("{c}"), "5");
    }

    #[test]
    fn test_complex_display_imaginary_only() {
        let c = Complex::new(0.0, 3.0);
        assert_eq!(format!("{c}"), "3i");
    }

    #[test]
    fn test_complex_display_imaginary_one() {
        let c = Complex::new(0.0, 1.0);
        assert_eq!(format!("{c}"), "i");
    }

    #[test]
    fn test_complex_display_imaginary_negative_one() {
        let c = Complex::new(0.0, -1.0);
        assert_eq!(format!("{c}"), "-i");
    }

    #[test]
    fn test_complex_display_positive_both() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(format!("{c}"), "3 + 4i");
    }

    #[test]
    fn test_complex_display_negative_imaginary() {
        let c = Complex::new(3.0, -4.0);
        assert_eq!(format!("{c}"), "3 - 4i");
    }

    #[test]
    fn test_complex_display_with_positive_one() {
        let c = Complex::new(2.0, 1.0);
        assert_eq!(format!("{c}"), "2 + i");
    }

    #[test]
    fn test_complex_display_with_negative_one() {
        let c = Complex::new(2.0, -1.0);
        assert_eq!(format!("{c}"), "2 - i");
    }

    #[test]
    fn test_complex_display_negative_real_positive_imaginary() {
        let c = Complex::new(-3.0, 2.0);
        assert_eq!(format!("{c}"), "-3 + 2i");
    }

    #[test]
    fn test_complex_display_negative_real_negative_imaginary() {
        let c = Complex::new(-3.0, -2.0);
        assert_eq!(format!("{c}"), "-3 - 2i");
    }

    #[test]
    fn test_complex_display_floats() {
        let c = Complex::new(1.5, 2.7);
        assert_eq!(format!("{c}"), "1.5 + 2.7i");
    }
}
