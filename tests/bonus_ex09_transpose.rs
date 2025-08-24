use linear_algebra_42::{Complex, Matrix};

#[test]
fn test_transpose_complex_matrix() {
    let m = Matrix::from([
        [Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)],
        [Complex::new(5.0, -1.0), Complex::new(0.0, 1.0)],
    ]);

    let transposed = m.transpose();

    assert_eq!(
        transposed,
        Matrix::from([
            [Complex::new(1.0, 2.0), Complex::new(5.0, -1.0)],
            [Complex::new(3.0, 4.0), Complex::new(0.0, 1.0)],
        ])
    );
}
