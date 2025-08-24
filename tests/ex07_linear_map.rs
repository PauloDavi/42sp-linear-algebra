use linear_algebra_42::{Matrix, Vector};

#[test]
fn test_mul_matrix_vector() {
    let a = Matrix::from([[1.0, 0.0], [0.0, 1.0]]);
    let v = Vector::from([4.0, 2.0]);
    let result = a.mul_vec(&v);
    assert_eq!(result.to_vec(), [4.0, 2.0]);

    let a = Matrix::from([[2.0, 0.0], [0.0, 2.0]]);
    let result = a.mul_vec(&v);
    assert_eq!(result.to_vec(), [8.0, 4.0]);

    let a = Matrix::from([[2.0, -2.0], [-2.0, 2.0]]);
    let result = a.mul_vec(&v);
    assert_eq!(result.to_vec(), [4.0, -4.0]);
}

#[test]
fn test_mul_matrix_matrix() {
    let a = Matrix::from([[1.0, 0.0], [0.0, 1.0]]);
    let b = Matrix::from([[2.0, 1.0], [4.0, 2.0]]);
    let result = a.mul_mat(&b);
    assert_eq!(result, Matrix::from([[2.0, 1.0], [4.0, 2.0]]));

    let a = Matrix::from([[3.0, -5.0], [6.0, 8.0]]);
    let b = Matrix::from([[2.0, 1.0], [4.0, 2.0]]);
    let result = a.mul_mat(&b);
    assert_eq!(result, Matrix::from([[-14.0, -7.0], [44.0, 22.0]]));
}
