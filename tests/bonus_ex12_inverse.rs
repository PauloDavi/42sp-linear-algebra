use linear_algebra_42::{Complex, Matrix};

#[test]
fn test_inverse_complex_matrix() {
    let m = Matrix::from([
        [Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)],
        [Complex::new(3.0, 0.0), Complex::new(4.0, 0.0)],
    ]);

    let inverse = m.inverse().expect("Matrix should be invertible");

    // Multiplicação m * inverse deve dar identidade
    let product = m.mul_mat(&inverse);
    let identity = Matrix::from([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]);

    for (row_r, row_e) in product.iter().zip(identity.iter()) {
        for (val_r, val_e) in row_r.iter().zip(row_e.iter()) {
            assert!((val_r.real() - val_e.real()).abs() < 1e-3);
            assert!((val_r.imaginary() - val_e.imaginary()).abs() < 1e-3);
        }
    }
}
