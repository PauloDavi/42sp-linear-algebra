use linear_algebra::{errors::MatrixError, matrix::Matrix};

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn test_new_success() {
        let data = [&[1, 2, 3][..], &[4, 5, 6][..], &[7, 8, 9][..]];
        let matrix = Matrix::new(&data);

        assert!(matrix.is_ok());
        let matrix = matrix.unwrap();
        assert_eq!(matrix.shape(), (3, 3));
        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[2][2], 9);
    }

    #[test]
    fn test_new_empty_matrix() {
        let data: [&[i32]; 0] = [];
        let matrix = Matrix::new(&data);

        assert!(matrix.is_ok());
        let matrix = matrix.unwrap();
        assert_eq!(matrix.shape(), (0, 0));
    }

    #[test]
    fn test_new_single_row() {
        let data = [&[1, 2, 3][..]];
        let matrix = Matrix::new(&data);

        assert!(matrix.is_ok());
        let matrix = matrix.unwrap();
        assert_eq!(matrix.shape(), (1, 3));
        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[0][2], 3);
    }

    #[test]
    fn test_new_single_column() {
        let data = [&[1][..], &[2][..], &[3][..]];
        let matrix = Matrix::new(&data);

        assert!(matrix.is_ok());
        let matrix = matrix.unwrap();
        assert_eq!(matrix.shape(), (3, 1));
        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[2][0], 3);
    }

    #[test]
    fn test_new_rows_length_mismatch() {
        let data = [&[1, 2, 3][..], &[4, 5][..], &[7, 8, 9][..]];
        let matrix = Matrix::new(&data);

        assert!(matrix.is_err());
        assert!(matches!(
            matrix.unwrap_err(),
            MatrixError::RowsLengthMismatch
        ));
    }

    #[test]
    fn test_shape() {
        let data = [&[1, 2][..], &[3, 4][..], &[5, 6][..]];
        let matrix = Matrix::new(&data).unwrap();

        assert_eq!(matrix.shape(), (3, 2));
    }

    #[test]
    fn test_add_success() {
        let data1 = [&[1, 2][..], &[3, 4][..]];
        let data2 = [&[5, 6][..], &[7, 8][..]];
        let mut matrix1 = Matrix::new(&data1).unwrap();
        let matrix2 = Matrix::new(&data2).unwrap();

        let result = matrix1.add(&matrix2);

        assert!(result.is_ok());
        assert_eq!(matrix1[0][0], 6);
        assert_eq!(matrix1[0][1], 8);
        assert_eq!(matrix1[1][0], 10);
        assert_eq!(matrix1[1][1], 12);
    }

    #[test]
    fn test_add_dimension_mismatch() {
        let data1 = [&[1, 2][..], &[3, 4][..]];
        let data2 = [&[5, 6, 7][..], &[8, 9, 10][..]];
        let mut matrix1 = Matrix::new(&data1).unwrap();
        let matrix2 = Matrix::new(&data2).unwrap();

        let result = matrix1.add(&matrix2);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MatrixError::DimensionMismatch
        ));
    }

    #[test]
    fn test_sub_success() {
        let data1 = [&[10, 8][..], &[6, 4][..]];
        let data2 = [&[5, 3][..], &[2, 1][..]];
        let mut matrix1 = Matrix::new(&data1).unwrap();
        let matrix2 = Matrix::new(&data2).unwrap();

        let result = matrix1.sub(&matrix2);

        assert!(result.is_ok());
        assert_eq!(matrix1[0][0], 5);
        assert_eq!(matrix1[0][1], 5);
        assert_eq!(matrix1[1][0], 4);
        assert_eq!(matrix1[1][1], 3);
    }

    #[test]
    fn test_sub_dimension_mismatch() {
        let data1 = [&[1, 2][..], &[3, 4][..]];
        let data2 = [&[5][..], &[6][..]];
        let mut matrix1 = Matrix::new(&data1).unwrap();
        let matrix2 = Matrix::new(&data2).unwrap();

        let result = matrix1.sub(&matrix2);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MatrixError::DimensionMismatch
        ));
    }

    #[test]
    fn test_scalar() {
        let data = [&[2, 3][..], &[4, 5][..]];
        let mut matrix = Matrix::new(&data).unwrap();

        matrix.scalar(3);

        assert_eq!(matrix[0][0], 6);
        assert_eq!(matrix[0][1], 9);
        assert_eq!(matrix[1][0], 12);
        assert_eq!(matrix[1][1], 15);
    }

    #[test]
    fn test_add_new_success() {
        let data1 = [&[1, 2][..], &[3, 4][..]];
        let data2 = [&[5, 6][..], &[7, 8][..]];
        let matrix1 = Matrix::new(&data1).unwrap();
        let matrix2 = Matrix::new(&data2).unwrap();

        let result = matrix1.add_new(&matrix2);

        assert!(result.is_ok());
        let new_matrix = result.unwrap();
        assert_eq!(new_matrix[0][0], 6);
        assert_eq!(new_matrix[0][1], 8);
        assert_eq!(new_matrix[1][0], 10);
        assert_eq!(new_matrix[1][1], 12);

        assert_eq!(matrix1[0][0], 1);
        assert_eq!(matrix2[0][0], 5);
    }

    #[test]
    fn test_add_new_dimension_mismatch() {
        let data1 = [&[1, 2][..], &[3, 4][..]];
        let data2 = [&[5, 6, 7][..]];
        let matrix1 = Matrix::new(&data1).unwrap();
        let matrix2 = Matrix::new(&data2).unwrap();

        let result = matrix1.add_new(&matrix2);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MatrixError::DimensionMismatch
        ));
    }

    #[test]
    fn test_sub_new_success() {
        let data1 = [&[10, 8][..], &[6, 4][..]];
        let data2 = [&[5, 3][..], &[2, 1][..]];
        let matrix1 = Matrix::new(&data1).unwrap();
        let matrix2 = Matrix::new(&data2).unwrap();

        let result = matrix1.sub_new(&matrix2);

        assert!(result.is_ok());
        let new_matrix = result.unwrap();
        assert_eq!(new_matrix[0][0], 5);
        assert_eq!(new_matrix[0][1], 5);
        assert_eq!(new_matrix[1][0], 4);
        assert_eq!(new_matrix[1][1], 3);

        assert_eq!(matrix1[0][0], 10);
        assert_eq!(matrix2[0][0], 5);
    }

    #[test]
    fn test_sub_new_dimension_mismatch() {
        let data1 = [&[1, 2][..], &[3, 4][..]];
        let data2 = [&[5, 6, 7][..], &[8, 9, 10][..], &[11, 12, 13][..]];
        let matrix1 = Matrix::new(&data1).unwrap();
        let matrix2 = Matrix::new(&data2).unwrap();

        let result = matrix1.sub_new(&matrix2);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MatrixError::DimensionMismatch
        ));
    }

    #[test]
    fn test_scalar_new() {
        let data = [&[2, 3][..], &[4, 5][..]];
        let matrix = Matrix::new(&data).unwrap();

        let new_matrix = matrix.scalar_new(3);

        assert_eq!(new_matrix[0][0], 6);
        assert_eq!(new_matrix[0][1], 9);
        assert_eq!(new_matrix[1][0], 12);
        assert_eq!(new_matrix[1][1], 15);

        // Verify original matrix is unchanged
        assert_eq!(matrix[0][0], 2);
        assert_eq!(matrix[0][1], 3);
        assert_eq!(matrix[1][0], 4);
        assert_eq!(matrix[1][1], 5);
    }

    #[test]
    fn test_index() {
        let data = [&[1, 2, 3][..], &[4, 5, 6][..], &[7, 8, 9][..]];
        let matrix = Matrix::new(&data).unwrap();

        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[0][2], 3);
        assert_eq!(matrix[1][1], 5);
        assert_eq!(matrix[2][2], 9);
    }

    #[test]
    fn test_index_mut() {
        let data = [&[1, 2][..], &[3, 4][..]];
        let mut matrix = Matrix::new(&data).unwrap();

        matrix[0][1] = 10;
        matrix[1][0] = 20;

        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[0][1], 10);
        assert_eq!(matrix[1][0], 20);
        assert_eq!(matrix[1][1], 4);
    }

    #[test]
    fn test_clone() {
        let data = [&[1, 2][..], &[3, 4][..]];
        let matrix1 = Matrix::new(&data).unwrap();
        let matrix2 = matrix1.clone();

        assert_eq!(matrix1, matrix2);
        assert_eq!(matrix1[0][0], matrix2[0][0]);
        assert_eq!(matrix1[1][1], matrix2[1][1]);
    }

    #[test]
    fn test_partial_eq() {
        let data1 = [&[1, 2][..], &[3, 4][..]];
        let data2 = [&[1, 2][..], &[3, 4][..]];
        let data3 = [&[1, 2][..], &[3, 5][..]];

        let matrix1 = Matrix::new(&data1).unwrap();
        let matrix2 = Matrix::new(&data2).unwrap();
        let matrix3 = Matrix::new(&data3).unwrap();

        assert_eq!(matrix1, matrix2);
        assert_ne!(matrix1, matrix3);
    }

    #[test]
    fn test_with_floats() {
        let data1 = [&[1.5, 2.5][..], &[3.5, 4.5][..]];
        let data2 = [&[0.5, 1.5][..], &[2.5, 3.5][..]];
        let matrix1 = Matrix::new(&data1).unwrap();
        let matrix2 = Matrix::new(&data2).unwrap();

        let result = matrix1.add_new(&matrix2).unwrap();

        assert_eq!(result[0][0], 2.0);
        assert_eq!(result[0][1], 4.0);
        assert_eq!(result[1][0], 6.0);
        assert_eq!(result[1][1], 8.0);
    }

    #[test]
    fn test_rectangular_matrix() {
        let data = [&[1, 2, 3, 4][..], &[5, 6, 7, 8][..]];
        let matrix = Matrix::new(&data).unwrap();

        assert_eq!(matrix.shape(), (2, 4));
        assert_eq!(matrix[0][3], 4);
        assert_eq!(matrix[1][3], 8);
    }

    #[test]
    fn test_single_element_matrix() {
        let data = [&[42][..]];
        let matrix = Matrix::new(&data).unwrap();

        assert_eq!(matrix.shape(), (1, 1));
        assert_eq!(matrix[0][0], 42);
    }

    #[test]
    fn test_scalar_with_zero() {
        let data = [&[1, 2][..], &[3, 4][..]];
        let mut matrix = Matrix::new(&data).unwrap();

        matrix.scalar(0);

        assert_eq!(matrix[0][0], 0);
        assert_eq!(matrix[0][1], 0);
        assert_eq!(matrix[1][0], 0);
        assert_eq!(matrix[1][1], 0);
    }

    #[test]
    fn test_scalar_with_negative() {
        let data = [&[1, 2][..], &[3, 4][..]];
        let matrix = Matrix::new(&data).unwrap();

        let new_matrix = matrix.scalar_new(-1);

        assert_eq!(new_matrix[0][0], -1);
        assert_eq!(new_matrix[0][1], -2);
        assert_eq!(new_matrix[1][0], -3);
        assert_eq!(new_matrix[1][1], -4);
    }
}
