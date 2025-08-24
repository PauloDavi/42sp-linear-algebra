use linear_algebra_42::{Complex, Matrix};

#[test]
fn test_determinant_complex_matrix_2x2() {
    let m = Matrix::from([
        [Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)],
        [Complex::new(2.0, 0.0), Complex::new(0.0, 1.0)],
    ]);

    let det = m.determinant();
    assert!((det.real() + 8.0).abs() < 1e-4);
    assert!((det.imaginary() - 3.0).abs() < 1e-4);
}
