use linear_algebra_42::{Complex, Vector};

#[test]
fn test_vector_complex_dot() {
    let a = Vector::from(vec![Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)]);

    let b = Vector::from(vec![Complex::new(4.0, 1.0), Complex::new(-2.0, 2.0)]);

    let dot = a.dot(&b);
    assert_eq!(dot, Complex::new(-2.0, 17.0));
}
