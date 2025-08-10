#[cfg(test)]
mod matrix_tests {
    use linear_algebra_42::{matrix::Matrix, vector::Vector};

    #[test]
    fn test_new_success() {
        let matrix = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        assert_eq!(matrix.shape(), (3, 3));
        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[2][2], 9);
    }

    #[test]
    fn test_new_single_row() {
        let data = [[1, 2, 3]];
        let matrix = Matrix::from(data);

        assert_eq!(matrix.shape(), (1, 3));
        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[0][2], 3);
    }

    #[test]
    fn test_new_single_column() {
        let data = [[1], [2], [3]];
        let matrix = Matrix::from(data);

        assert_eq!(matrix.shape(), (3, 1));
        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[2][0], 3);
    }

    #[test]
    fn test_shape() {
        let data = [[1, 2], [3, 4], [5, 6]];
        let matrix = Matrix::from(data);

        assert_eq!(matrix.shape(), (3, 2));
    }

    #[test]
    fn test_add_success() {
        let data1 = [[1, 2], [3, 4]];
        let data2 = [[5, 6], [7, 8]];
        let mut matrix1 = Matrix::from(data1);
        let matrix2 = Matrix::from(data2);

        matrix1.add(&matrix2);

        assert_eq!(matrix1[0][0], 6);
        assert_eq!(matrix1[0][1], 8);
        assert_eq!(matrix1[1][0], 10);
        assert_eq!(matrix1[1][1], 12);
    }

    #[test]
    fn test_sub_success() {
        let data1 = [[10, 8], [6, 4]];
        let data2 = [[5, 3], [2, 1]];
        let mut matrix1 = Matrix::from(data1);
        let matrix2 = Matrix::from(data2);

        matrix1.sub(&matrix2);

        assert_eq!(matrix1[0][0], 5);
        assert_eq!(matrix1[0][1], 5);
        assert_eq!(matrix1[1][0], 4);
        assert_eq!(matrix1[1][1], 3);
    }

    #[test]
    fn test_scl() {
        let data = [[2, 3], [4, 5]];
        let mut matrix = Matrix::from(data);

        matrix.scl(3);

        assert_eq!(matrix[0][0], 6);
        assert_eq!(matrix[0][1], 9);
        assert_eq!(matrix[1][0], 12);
        assert_eq!(matrix[1][1], 15);
    }

    #[test]
    fn test_add_new_success() {
        let data1 = [[1, 2], [3, 4]];
        let data2 = [[5, 6], [7, 8]];
        let matrix1 = Matrix::from(data1);
        let matrix2 = Matrix::from(data2);

        let new_matrix = matrix1.add_new(&matrix2);

        assert_eq!(new_matrix[0][0], 6);
        assert_eq!(new_matrix[0][1], 8);
        assert_eq!(new_matrix[1][0], 10);
        assert_eq!(new_matrix[1][1], 12);

        assert_eq!(matrix1[0][0], 1);
        assert_eq!(matrix2[0][0], 5);
    }

    #[test]
    fn test_sub_new_success() {
        let data1 = [[10, 8], [6, 4]];
        let data2 = [[5, 3], [2, 1]];
        let matrix1 = Matrix::from(data1);
        let matrix2 = Matrix::from(data2);

        let new_matrix = matrix1.sub_new(&matrix2);

        assert_eq!(new_matrix[0][0], 5);
        assert_eq!(new_matrix[0][1], 5);
        assert_eq!(new_matrix[1][0], 4);
        assert_eq!(new_matrix[1][1], 3);

        assert_eq!(matrix1[0][0], 10);
        assert_eq!(matrix2[0][0], 5);
    }

    #[test]
    fn test_scl_new() {
        let data = [[2, 3], [4, 5]];
        let matrix = Matrix::from(data);

        let new_matrix = matrix.scl_new(3);

        assert_eq!(new_matrix[0][0], 6);
        assert_eq!(new_matrix[0][1], 9);
        assert_eq!(new_matrix[1][0], 12);
        assert_eq!(new_matrix[1][1], 15);

        assert_eq!(matrix[0][0], 2);
        assert_eq!(matrix[0][1], 3);
        assert_eq!(matrix[1][0], 4);
        assert_eq!(matrix[1][1], 5);
    }

    #[test]
    fn test_index() {
        let data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let matrix = Matrix::from(data);

        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[0][2], 3);
        assert_eq!(matrix[1][1], 5);
        assert_eq!(matrix[2][2], 9);
    }

    #[test]
    fn test_index_mut() {
        let data = [[1, 2], [3, 4]];
        let mut matrix = Matrix::from(data);

        matrix[0][1] = 10;
        matrix[1][0] = 20;

        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[0][1], 10);
        assert_eq!(matrix[1][0], 20);
        assert_eq!(matrix[1][1], 4);
    }

    #[test]
    fn test_clone() {
        let data = [[1, 2], [3, 4]];
        let matrix1 = Matrix::from(data);
        let matrix2 = matrix1.clone();

        assert_eq!(matrix1, matrix2);
        assert_eq!(matrix1[0][0], matrix2[0][0]);
        assert_eq!(matrix1[1][1], matrix2[1][1]);
    }

    #[test]
    fn test_partial_eq() {
        let data1 = [[1, 2], [3, 4]];
        let data2 = [[1, 2], [3, 4]];
        let data3 = [[1, 2], [3, 5]];

        let matrix1 = Matrix::from(data1);
        let matrix2 = Matrix::from(data2);
        let matrix3 = Matrix::from(data3);

        assert_eq!(matrix1, matrix2);
        assert_ne!(matrix1, matrix3);
    }

    #[test]
    fn test_with_floats() {
        let data1 = [[1.5, 2.5], [3.5, 4.5]];
        let data2 = [[0.5, 1.5], [2.5, 3.5]];
        let matrix1 = Matrix::from(data1);
        let matrix2 = Matrix::from(data2);

        let result = matrix1.add_new(&matrix2);

        assert_eq!(result[0][0], 2.0);
        assert_eq!(result[0][1], 4.0);
        assert_eq!(result[1][0], 6.0);
        assert_eq!(result[1][1], 8.0);
    }

    #[test]
    fn test_rectangular_matrix() {
        let data = [[1, 2, 3, 4], [5, 6, 7, 8]];
        let matrix = Matrix::from(data);

        assert_eq!(matrix.shape(), (2, 4));
        assert_eq!(matrix[0][3], 4);
        assert_eq!(matrix[1][3], 8);
    }

    #[test]
    fn test_single_element_matrix() {
        let data = [[42]];
        let matrix = Matrix::from(data);

        assert_eq!(matrix.shape(), (1, 1));
        assert_eq!(matrix[0][0], 42);
    }

    #[test]
    fn test_scl_with_zero() {
        let data = [[1, 2], [3, 4]];
        let mut matrix = Matrix::from(data);

        matrix.scl(0);

        assert_eq!(matrix[0][0], 0);
        assert_eq!(matrix[0][1], 0);
        assert_eq!(matrix[1][0], 0);
        assert_eq!(matrix[1][1], 0);
    }

    #[test]
    fn test_scl_with_negative() {
        let data = [[1, 2], [3, 4]];
        let matrix = Matrix::from(data);

        let new_matrix = matrix.scl_new(-1);

        assert_eq!(new_matrix[0][0], -1);
        assert_eq!(new_matrix[0][1], -2);
        assert_eq!(new_matrix[1][0], -3);
        assert_eq!(new_matrix[1][1], -4);
    }

    #[test]
    fn test_zeros() {
        let matrix: Matrix<i32> = Matrix::zeros(3, 2);

        assert_eq!(matrix.shape(), (3, 2));
        assert_eq!(matrix[0][0], 0);
        assert_eq!(matrix[0][1], 0);
        assert_eq!(matrix[1][0], 0);
        assert_eq!(matrix[1][1], 0);
        assert_eq!(matrix[2][0], 0);
        assert_eq!(matrix[2][1], 0);
    }

    #[test]
    fn test_zeros_floats() {
        let matrix: Matrix<f64> = Matrix::zeros(2, 3);

        assert_eq!(matrix.shape(), (2, 3));
        assert_eq!(matrix[0][0], 0.0);
        assert_eq!(matrix[1][2], 0.0);
    }

    #[test]
    fn test_transpose_square() {
        let data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let matrix = Matrix::from(data);

        let transposed = matrix.transpose();

        assert_eq!(transposed.shape(), (3, 3));
        assert_eq!(transposed[0][0], 1);
        assert_eq!(transposed[0][1], 4);
        assert_eq!(transposed[0][2], 7);
        assert_eq!(transposed[1][0], 2);
        assert_eq!(transposed[1][1], 5);
        assert_eq!(transposed[1][2], 8);
        assert_eq!(transposed[2][0], 3);
        assert_eq!(transposed[2][1], 6);
        assert_eq!(transposed[2][2], 9);
    }

    #[test]
    fn test_transpose_rectangular() {
        let data = [[1, 2, 3], [4, 5, 6]];
        let matrix = Matrix::from(data);

        let transposed = matrix.transpose();

        assert_eq!(transposed.shape(), (3, 2));
        assert_eq!(transposed[0][0], 1);
        assert_eq!(transposed[0][1], 4);
        assert_eq!(transposed[1][0], 2);
        assert_eq!(transposed[1][1], 5);
        assert_eq!(transposed[2][0], 3);
        assert_eq!(transposed[2][1], 6);
    }

    #[test]
    fn test_transpose_single_row() {
        let data = [[1, 2, 3, 4]];
        let matrix = Matrix::from(data);

        let transposed = matrix.transpose();

        assert_eq!(transposed.shape(), (4, 1));
        assert_eq!(transposed[0][0], 1);
        assert_eq!(transposed[1][0], 2);
        assert_eq!(transposed[2][0], 3);
        assert_eq!(transposed[3][0], 4);
    }

    #[test]
    fn test_iter() {
        let data = [[1, 2], [3, 4]];
        let matrix = Matrix::from(data);

        let mut iter = matrix.iter();
        let first_row = iter.next().unwrap();
        let second_row = iter.next().unwrap();

        assert_eq!(first_row, &vec![1, 2]);
        assert_eq!(second_row, &vec![3, 4]);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iter_mut() {
        let data = [[1, 2], [3, 4]];
        let mut matrix = Matrix::from(data);

        for row in matrix.iter_mut() {
            for element in row.iter_mut() {
                *element *= 2;
            }
        }

        assert_eq!(matrix[0][0], 2);
        assert_eq!(matrix[0][1], 4);
        assert_eq!(matrix[1][0], 6);
        assert_eq!(matrix[1][1], 8);
    }

    #[test]
    fn test_mul_vec_identity() {
        let matrix = Matrix::from([[1.0, 0.0], [0.0, 1.0]]);
        let vector = Vector::from([3.0, 4.0]);

        let result = matrix.mul_vec(&vector);

        assert_eq!(result[0], 3.0);
        assert_eq!(result[1], 4.0);
    }

    #[test]
    fn test_mul_vec_general() {
        let matrix = Matrix::from([[2.0, 3.0], [4.0, 5.0]]);
        let vector = Vector::from([1.0, 2.0]);

        let result = matrix.mul_vec(&vector);

        assert_eq!(result[0], 10.0);
        assert_eq!(result[1], 13.0);
    }

    #[test]
    fn test_mul_mat_square() {
        let matrix1 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let matrix2 = Matrix::from([[5.0, 6.0], [7.0, 8.0]]);

        let result = matrix1.mul_mat(&matrix2);

        assert_eq!(result.shape().0, 2);
        assert_eq!(result.shape().1, 2);
        assert!(result[0][0] != 0.0 || result[0][1] != 0.0);
    }

    #[test]
    fn test_mul_mat_identity() {
        let matrix1 = Matrix::from([[1.0, 0.0], [0.0, 1.0]]);
        let matrix2 = Matrix::from([[2.0, 3.0], [4.0, 5.0]]);

        let result = matrix1.mul_mat(&matrix2);

        assert_eq!(result[0][0], 2.0);
        assert_eq!(result[0][1], 3.0);
        assert_eq!(result[1][0], 4.0);
        assert_eq!(result[1][1], 5.0);
    }

    #[test]
    fn test_mul_mat_general() {
        let matrix1 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let matrix2 = Matrix::from([[5.0, 6.0], [7.0, 8.0]]);

        let result = matrix1.mul_mat(&matrix2);

        assert_eq!(result[0][0], 19.0);
        assert_eq!(result[0][1], 22.0);
        assert_eq!(result[1][0], 43.0);
        assert_eq!(result[1][1], 50.0);
    }

    #[test]
    fn test_add_trait() {
        let matrix1 = Matrix::from([[1, 2], [3, 4]]);
        let matrix2 = Matrix::from([[5, 6], [7, 8]]);

        let result = matrix1 + matrix2;

        assert_eq!(result[0][0], 6);
        assert_eq!(result[0][1], 8);
        assert_eq!(result[1][0], 10);
        assert_eq!(result[1][1], 12);
    }

    #[test]
    fn test_sub_trait() {
        let matrix1 = Matrix::from([[10, 8], [6, 4]]);
        let matrix2 = Matrix::from([[5, 3], [2, 1]]);

        let result = matrix1 - matrix2;

        assert_eq!(result[0][0], 5);
        assert_eq!(result[0][1], 5);
        assert_eq!(result[1][0], 4);
        assert_eq!(result[1][1], 3);
    }

    #[test]
    fn test_mul_scalar_trait() {
        let matrix = Matrix::from([[2, 3], [4, 5]]);

        let result = matrix * 3;

        assert_eq!(result[0][0], 6);
        assert_eq!(result[0][1], 9);
        assert_eq!(result[1][0], 12);
        assert_eq!(result[1][1], 15);
    }

    #[test]
    fn test_transpose_double() {
        let data = [[1, 2, 3], [4, 5, 6]];
        let matrix = Matrix::from(data);

        let double_transposed = matrix.transpose().transpose();

        assert_eq!(matrix, double_transposed);
    }

    #[test]
    fn test_trace_square_matrix() {
        let matrix = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        let trace = matrix.trace();

        assert_eq!(trace, 15);
    }

    #[test]
    fn test_trace_identity_matrix() {
        let matrix = Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);

        let trace = matrix.trace();

        assert_eq!(trace, 3);
    }

    #[test]
    fn test_trace_2x2_matrix() {
        let matrix = Matrix::from([[5, 2], [3, 7]]);

        let trace = matrix.trace();

        assert_eq!(trace, 12);
    }

    #[test]
    fn test_trace_single_element() {
        let matrix = Matrix::from([[42]]);

        let trace = matrix.trace();

        assert_eq!(trace, 42);
    }

    #[test]
    fn test_trace_with_floats() {
        let matrix = Matrix::from([[1.5, 2.0], [3.5, 4.5]]);

        let trace = matrix.trace();

        assert_eq!(trace, 6.0);
    }

    #[test]
    fn test_trace_with_negatives() {
        let matrix = Matrix::from([[-1, 2], [3, -4]]);

        let trace = matrix.trace();

        assert_eq!(trace, -5);
    }

    #[test]
    fn test_trace_rectangular_matrix() {
        let matrix = Matrix::from([[1, 2, 3], [4, 5, 6]]);

        let trace = matrix.trace();

        assert_eq!(trace, 0);
    }

    #[test]
    fn test_trace_zeros() {
        let matrix = Matrix::from([[0, 1, 2], [3, 0, 5], [6, 7, 0]]);

        let trace = matrix.trace();

        assert_eq!(trace, 0);
    }

    #[test]
    fn test_is_square() {
        let square_matrix = Matrix::from([[1, 2], [3, 4]]);
        let rectangular_matrix = Matrix::from([[1, 2, 3], [4, 5, 6]]);

        assert!(square_matrix.is_square());
        assert!(!rectangular_matrix.is_square());
    }

    #[test]
    fn test_row_echelon_simple_matrix() {
        let matrix = Matrix::from([[2.0_f64, 4.0, 6.0], [1.0, 3.0, 5.0], [0.0, 2.0, 4.0]]);

        let result = matrix.row_echelon();

        assert!((result[0][0] - 1.0).abs() < 1e-10);
        assert!(result[1][0].abs() < 1e-10);
        assert!(result[2][0].abs() < 1e-10);
    }

    #[test]
    fn test_row_echelon_identity_matrix() {
        let matrix = Matrix::from([[1.0_f64, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

        let result = matrix.row_echelon();

        assert_eq!(result, matrix);
    }

    #[test]
    fn test_row_echelon_with_zeros() {
        let matrix = Matrix::from([[0.0_f64, 2.0, 4.0], [1.0, 3.0, 5.0], [2.0, 4.0, 6.0]]);

        let result = matrix.row_echelon();

        assert!(result[0][0] != 0.0 || result[1][0] != 0.0 || result[2][0] != 0.0);
    }

    #[test]
    fn test_row_echelon_rectangular_matrix() {
        let matrix = Matrix::from([
            [1.0_f64, 2.0, 3.0, 4.0],
            [2.0, 4.0, 6.0, 8.0],
            [1.0, 1.0, 1.0, 1.0],
        ]);

        let result = matrix.row_echelon();

        assert_eq!(result.shape(), (3, 4));
        assert!((result[0][0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_row_echelon_with_negative_values() {
        let matrix = Matrix::from([[-2.0_f64, 4.0, -6.0], [1.0, -3.0, 5.0], [0.0, 2.0, -4.0]]);

        let result = matrix.row_echelon();

        assert_eq!(result.shape(), (3, 3));
        assert!((result[0][0].abs() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_sub_matrix_remove_first_row_first_col() {
        let matrix = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        let result = matrix.sub_matrix(0, 0);

        assert_eq!(result.shape(), (2, 2));
        assert_eq!(result[0][0], 5);
        assert_eq!(result[0][1], 6);
        assert_eq!(result[1][0], 8);
        assert_eq!(result[1][1], 9);
    }

    #[test]
    fn test_sub_matrix_remove_middle_row_middle_col() {
        let matrix = Matrix::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);

        let result = matrix.sub_matrix(1, 2);

        assert_eq!(result.shape(), (3, 3));
        assert_eq!(result[0][0], 1);
        assert_eq!(result[0][1], 2);
        assert_eq!(result[0][2], 4);
        assert_eq!(result[1][0], 9);
        assert_eq!(result[1][1], 10);
        assert_eq!(result[1][2], 12);
        assert_eq!(result[2][0], 13);
        assert_eq!(result[2][1], 14);
        assert_eq!(result[2][2], 16);
    }

    #[test]
    fn test_sub_matrix_remove_last_row_last_col() {
        let matrix = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        let result = matrix.sub_matrix(2, 2);

        assert_eq!(result.shape(), (2, 2));
        assert_eq!(result[0][0], 1);
        assert_eq!(result[0][1], 2);
        assert_eq!(result[1][0], 4);
        assert_eq!(result[1][1], 5);
    }

    #[test]
    fn test_sub_matrix_2x2_to_1x1() {
        let matrix = Matrix::from([[1, 2], [3, 4]]);

        let result = matrix.sub_matrix(0, 0);

        assert_eq!(result.shape(), (1, 1));
        assert_eq!(result[0][0], 4);
    }

    #[test]
    fn test_sub_matrix_rectangular() {
        let matrix = Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);

        let result = matrix.sub_matrix(1, 1);

        assert_eq!(result.shape(), (2, 3));
        assert_eq!(result[0][0], 1);
        assert_eq!(result[0][1], 3);
        assert_eq!(result[0][2], 4);
        assert_eq!(result[1][0], 9);
        assert_eq!(result[1][1], 11);
        assert_eq!(result[1][2], 12);
    }

    #[test]
    fn test_determinant_1x1() {
        let matrix = Matrix::from([[5]]);

        let det = matrix.determinant();

        assert_eq!(det, 5);
    }

    #[test]
    fn test_determinant_2x2_simple() {
        let matrix = Matrix::from([[1, 2], [3, 4]]);

        let det = matrix.determinant();

        assert_eq!(det, -2);
    }

    #[test]
    fn test_determinant_2x2_identity() {
        let matrix = Matrix::from([[1, 0], [0, 1]]);

        let det = matrix.determinant();

        assert_eq!(det, 1);
    }

    #[test]
    fn test_determinant_2x2_zero() {
        let matrix = Matrix::from([[1, 2], [2, 4]]);

        let det = matrix.determinant();

        assert_eq!(det, 0);
    }

    #[test]
    fn test_determinant_3x3_identity() {
        let matrix = Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);

        let det = matrix.determinant();

        assert_eq!(det, 1);
    }

    #[test]
    fn test_determinant_3x3_simple() {
        let matrix = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        let det = matrix.determinant();

        assert_eq!(det, 0);
    }

    #[test]
    fn test_determinant_3x3_non_singular() {
        let matrix = Matrix::from([[1, 2, 3], [0, 1, 4], [5, 6, 0]]);

        let det = matrix.determinant();

        assert_eq!(det, 1);
    }

    #[test]
    fn test_determinant_4x4_identity() {
        let matrix = Matrix::from([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);

        let det = matrix.determinant();

        assert_eq!(det, 1);
    }

    #[test]
    fn test_determinant_4x4_simple() {
        let matrix = Matrix::from([[2, 0, 0, 0], [0, 3, 0, 0], [0, 0, 4, 0], [0, 0, 0, 5]]);

        let det = matrix.determinant();

        assert_eq!(det, 120);
    }

    #[test]
    fn test_determinant_non_square() {
        let matrix = Matrix::from([[1, 2, 3], [4, 5, 6]]);

        let det = matrix.determinant();

        assert_eq!(det, 0);
    }

    #[test]
    fn test_determinant_with_floats() {
        let matrix = Matrix::from([[2.0, 1.0], [1.0, 2.0]]);

        let det = matrix.determinant();

        assert_eq!(det, 3.0);
    }

    #[test]
    fn test_determinant_with_negatives() {
        let matrix = Matrix::from([[-1, 2], [3, -4]]);

        let det = matrix.determinant();

        assert_eq!(det, -2);
    }

    #[test]
    fn test_determinant_larger_than_4x4() {
        let matrix = Matrix::from([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25],
        ]);

        let det = matrix.determinant();

        assert_eq!(det, 0);
    }

    #[test]
    fn test_row_echelon_all_zeros_row() {
        let matrix = Matrix::from([[1.0_f64, 2.0, 3.0], [0.0, 0.0, 0.0], [4.0, 5.0, 6.0]]);

        let result = matrix.row_echelon();

        assert_eq!(result.shape(), (3, 3));
        assert!(result[0][0] != 0.0 || result[2][0] != 0.0);
    }

    #[test]
    fn test_row_echelon_single_row() {
        let matrix = Matrix::from([[2.0_f64, 4.0, 6.0]]);

        let result = matrix.row_echelon();

        assert_eq!(result.shape(), (1, 3));
        assert!((result[0][0] - 1.0).abs() < 1e-10);
        assert!((result[0][1] - 2.0).abs() < 1e-10);
        assert!((result[0][2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_row_echelon_single_column() {
        let matrix = Matrix::from([[2.0_f64], [4.0], [6.0]]);

        let result = matrix.row_echelon();

        assert_eq!(result.shape(), (3, 1));
        assert!((result[0][0] - 1.0).abs() < 1e-10);
        assert!(result[1][0].abs() < 1e-10);
        assert!(result[2][0].abs() < 1e-10);
    }

    #[test]
    fn test_inverse_identity_2x2() {
        let matrix = Matrix::from([[1.0_f64, 0.0], [0.0, 1.0]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();
        assert_eq!(inv.shape(), (2, 2));
        assert!((inv[0][0] - 1.0).abs() < 1e-10);
        assert!(inv[0][1].abs() < 1e-10);
        assert!(inv[1][0].abs() < 1e-10);
        assert!((inv[1][1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_identity_3x3() {
        let matrix = Matrix::from([[1.0_f64, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();
        assert_eq!(inv, matrix);
    }

    #[test]
    fn test_inverse_simple_2x2() {
        let matrix = Matrix::from([[2.0_f64, 0.0], [0.0, 3.0]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();
        assert_eq!(inv.shape(), (2, 2));
        assert!((inv[0][0] - 0.5).abs() < 1e-10);
        assert!(inv[0][1].abs() < 1e-10);
        assert!(inv[1][0].abs() < 1e-10);
        assert!((inv[1][1] - 1.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_general_2x2() {
        let matrix = Matrix::from([[1.0_f64, 2.0], [3.0, 4.0]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();
        assert_eq!(inv.shape(), (2, 2));

        assert!((inv[0][0] - (-2.0)).abs() < 1e-10);
        assert!((inv[0][1] - 1.0).abs() < 1e-10);
        assert!((inv[1][0] - 1.5).abs() < 1e-10);
        assert!((inv[1][1] - (-0.5)).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_multiplication_identity() {
        let matrix = Matrix::from([[2.0_f64, 1.0], [1.0, 1.0]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();

        let product = matrix.mul_mat(&inv);
        assert!((product[0][0] - 1.0).abs() < 1e-10);
        assert!(product[0][1].abs() < 1e-10);
        assert!(product[1][0].abs() < 1e-10);
        assert!((product[1][1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_3x3_diagonal() {
        let matrix = Matrix::from([[2.0_f64, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();
        assert_eq!(inv.shape(), (3, 3));

        assert!((inv[0][0] - 0.5).abs() < 1e-10);
        assert!(inv[0][1].abs() < 1e-10);
        assert!(inv[0][2].abs() < 1e-10);
        assert!(inv[1][0].abs() < 1e-10);
        assert!((inv[1][1] - 1.0 / 3.0).abs() < 1e-10);
        assert!(inv[1][2].abs() < 1e-10);
        assert!(inv[2][0].abs() < 1e-10);
        assert!(inv[2][1].abs() < 1e-10);
        assert!((inv[2][2] - 0.25).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_3x3_general() {
        let matrix = Matrix::from([[1.0_f64, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();
        assert_eq!(inv.shape(), (3, 3));

        let product = matrix.mul_mat(&inv);
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (product[i][j] - expected).abs() < 1e-10,
                    "Element ({},{}) = {}, expected {}",
                    i,
                    j,
                    product[i][j],
                    expected
                );
            }
        }
    }

    #[test]
    fn test_inverse_singular_matrix() {
        let matrix = Matrix::from([[1.0_f64, 2.0], [2.0, 4.0]]);

        let result = matrix.inverse();

        assert!(result.is_err());
    }

    #[test]
    fn test_inverse_zero_diagonal() {
        let matrix = Matrix::from([[0.0_f64, 1.0], [1.0, 1.0]]);

        let result = matrix.inverse();

        assert!(result.is_err());
    }

    #[test]
    fn test_inverse_non_square_matrix() {
        let matrix = Matrix::from([[1.0_f64, 2.0, 3.0], [4.0, 5.0, 6.0]]);

        let result = matrix.inverse();

        assert!(result.is_err());
    }

    #[test]
    fn test_inverse_single_element() {
        let matrix = Matrix::from([[5.0_f64]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();
        assert_eq!(inv.shape(), (1, 1));
        assert!((inv[0][0] - 0.2).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_single_element_zero() {
        let matrix = Matrix::from([[0.0_f64]]);

        let result = matrix.inverse();

        assert!(result.is_err());
    }

    #[test]
    fn test_inverse_with_integers() {
        let matrix = Matrix::from([[1, 0], [0, 1]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();
        assert_eq!(inv[0][0], 1);
        assert_eq!(inv[0][1], 0);
        assert_eq!(inv[1][0], 0);
        assert_eq!(inv[1][1], 1);
    }

    #[test]
    fn test_inverse_with_negatives() {
        let matrix = Matrix::from([[-1.0_f64, 0.0], [0.0, -2.0]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();
        assert!((inv[0][0] - (-1.0)).abs() < 1e-10);
        assert!(inv[0][1].abs() < 1e-10);
        assert!(inv[1][0].abs() < 1e-10);
        assert!((inv[1][1] - (-0.5)).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_4x4_identity() {
        let matrix = Matrix::from([
            [1.0_f64, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();
        assert_eq!(inv, matrix);
    }

    #[test]
    fn test_inverse_symmetry() {
        let matrix = Matrix::from([[2.0_f64, 1.0, 0.0], [1.0, 2.0, 1.0], [0.0, 1.0, 2.0]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();

        let double_inv = inv.inverse().unwrap();
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (double_inv[i][j] - matrix[i][j]).abs() < 1e-10,
                    "Element ({},{}) = {}, expected {}",
                    i,
                    j,
                    double_inv[i][j],
                    matrix[i][j]
                );
            }
        }
    }

    #[test]
    fn test_inverse_precision() {
        let matrix = Matrix::from([[1.0000001_f64, 1.0], [1.0, 1.0000001]]);

        let result = matrix.inverse();

        assert!(result.is_ok());
        let inv = result.unwrap();

        let product = matrix.mul_mat(&inv);
        assert!((product[0][0] - 1.0).abs() < 1e-6);
        assert!(product[0][1].abs() < 1e-6);
        assert!(product[1][0].abs() < 1e-6);
        assert!((product[1][1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_rank_identity_2x2() {
        let mut matrix = Matrix::from([[1.0_f64, 0.0], [0.0, 1.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_identity_3x3() {
        let mut matrix = Matrix::from([[1.0_f64, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 3);
    }

    #[test]
    fn test_rank_full_rank_2x2() {
        let mut matrix = Matrix::from([[1.0_f64, 2.0], [3.0, 4.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_singular_2x2() {
        let mut matrix = Matrix::from([[1.0_f64, 2.0], [2.0, 4.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 1);
    }

    #[test]
    fn test_rank_zero_matrix() {
        let mut matrix = Matrix::from([[0.0_f64, 0.0], [0.0, 0.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 0);
    }

    #[test]
    fn test_rank_zero_row() {
        let mut matrix = Matrix::from([[1.0_f64, 2.0, 3.0], [0.0, 0.0, 0.0], [4.0, 5.0, 6.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_rectangular_full_row_rank() {
        let mut matrix = Matrix::from([[1.0_f64, 2.0, 3.0], [4.0, 5.0, 6.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_rectangular_full_column_rank() {
        let mut matrix = Matrix::from([[1.0_f64, 0.0], [0.0, 1.0], [1.0, 1.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_rectangular_reduced_rank() {
        let mut matrix = Matrix::from([[1.0_f64, 2.0, 3.0], [2.0, 4.0, 6.0], [4.0, 5.0, 6.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_single_row() {
        let mut matrix = Matrix::from([[1.0_f64, 2.0, 3.0, 4.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 1);
    }

    #[test]
    fn test_rank_single_column() {
        let mut matrix = Matrix::from([[1.0_f64], [2.0], [3.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 1);
    }

    #[test]
    fn test_rank_single_element() {
        let mut matrix = Matrix::from([[5.0_f64]]);

        let rank = matrix.rank();

        assert_eq!(rank, 1);
    }

    #[test]
    fn test_rank_single_element_zero() {
        let mut matrix = Matrix::from([[0.0_f64]]);

        let rank = matrix.rank();

        assert_eq!(rank, 0);
    }

    #[test]
    fn test_rank_diagonal_matrix() {
        let mut matrix = Matrix::from([[2.0_f64, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 3);
    }

    #[test]
    fn test_rank_diagonal_with_zero() {
        let mut matrix = Matrix::from([[2.0_f64, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 4.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_upper_triangular() {
        let mut matrix = Matrix::from([[1.0_f64, 2.0, 3.0], [0.0, 4.0, 5.0], [0.0, 0.0, 6.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 3);
    }

    #[test]
    fn test_rank_upper_triangular_with_zero() {
        let mut matrix = Matrix::from([[1.0_f64, 2.0, 3.0], [0.0, 4.0, 5.0], [0.0, 0.0, 0.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_all_same_rows() {
        let mut matrix = Matrix::from([[1.0_f64, 2.0, 3.0], [1.0, 2.0, 3.0], [1.0, 2.0, 3.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 1);
    }

    #[test]
    fn test_rank_complex_example() {
        let mut matrix = Matrix::from([
            [1.0_f64, 2.0, 1.0, 3.0],
            [2.0, 4.0, 3.0, 7.0],
            [1.0, 2.0, 2.0, 4.0],
            [3.0, 6.0, 4.0, 10.0],
        ]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_nearly_zero_elements() {
        let mut matrix =
            Matrix::from([[1.0_f64, 2.0, 3.0], [1e-15, 1e-14, 1e-13], [4.0, 5.0, 6.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_precision_boundary() {
        let mut matrix = Matrix::from([[1.0_f64, 2.0], [1e-11, 2e-11]]);

        let rank = matrix.rank();

        assert_eq!(rank, 1);
    }

    #[test]
    fn test_rank_large_matrix() {
        let mut matrix = Matrix::from([
            [1.0_f64, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0],
        ]);

        let rank = matrix.rank();

        assert_eq!(rank, 5);
    }

    #[test]
    fn test_rank_negative_values() {
        let mut matrix = Matrix::from([[-1.0_f64, 2.0, -3.0], [4.0, -5.0, 6.0], [-7.0, 8.0, -9.0]]);

        let rank = matrix.rank();

        assert_eq!(rank, 2);
    }

    #[test]
    fn test_rank_mutation_check() {
        let original = Matrix::from([[1.0_f64, 2.0], [3.0, 4.0]]);
        let mut matrix = original.clone();

        let rank = matrix.rank();

        assert_eq!(rank, 2);
        assert_ne!(matrix, original);
    }
}
