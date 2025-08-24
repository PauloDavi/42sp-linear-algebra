use linear_algebra_42::{angle_cos, Complex, Vector};

#[test]
fn test_cosine_similarity_complex() {
    let u = Vector::from(vec![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)]);

    let v = Vector::from(vec![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)]);

    let sim = angle_cos(&u, &v);
    assert!((sim - 1.0).abs() < 1e-6);

    let u = Vector::from(vec![Complex::new(1.0, 1.0), Complex::new(0.0, 0.0)]);

    let v = Vector::from(vec![Complex::new(1.0, -1.0), Complex::new(0.0, 0.0)]);

    let sim = angle_cos(&u, &v);
    assert!((sim - 1.0).abs() < 1e-6);
}
