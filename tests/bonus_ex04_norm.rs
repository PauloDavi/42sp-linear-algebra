use linear_algebra_42::{Complex, Vector};

#[test]
fn test_norms_complex_vector() {
    let v = Vector::from(vec![
        Complex::new(3.0, 4.0), // |z| = 5
        Complex::new(1.0, 0.0), // |z| = 1
        Complex::new(0.0, 2.0), // |z| = 2
    ]);

    let norm_1 = v.norm_1(); // 5 + 1 + 2 = 8
    let norm_2 = v.norm(); // sqrt(25 + 1 + 4) = sqrt(30) ≈ 5.477
    let norm_inf = v.norm_inf(); // max(5, 1, 2) = 5

    assert!((norm_1 - 8.0).abs() < 1e-4);
    assert!((norm_2 - 5.4772).abs() < 1e-4);
    assert!((norm_inf - 5.0).abs() < 1e-4);
}
