use linear_algebra_42::{Complex, Matrix};

#[test]
fn test_row_echelon_complex_matrix() {
    let m = Matrix::from([
        [Complex::new(2.0, 0.0), Complex::new(4.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::new(3.0, 0.0)],
    ]);

    let echelon = m.row_echelon();

    let expected = [
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ];

    for (row_e, row_r) in expected.iter().zip(echelon.iter()) {
        for (val_e, val_r) in row_e.iter().zip(row_r.iter()) {
            assert!((val_e.real() - val_r.real()).abs() < 1e-4);
            assert!((val_e.imaginary() - val_r.imaginary()).abs() < 1e-4);
        }
    }
}
