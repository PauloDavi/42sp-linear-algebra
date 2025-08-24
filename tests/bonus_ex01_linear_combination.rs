use linear_algebra_42::{linear_combination, Complex, Vector};

#[test]
fn test_vector_complex_linear_combination() {
    let v1 = Vector::from(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 1.0)]);
    let v2 = Vector::from(vec![Complex::new(0.0, 1.0), Complex::new(1.0, 0.0)]);

    let result =
        linear_combination([v1, v2], [Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)]).unwrap();

    assert_eq!(
        result.to_vec(),
        vec![Complex::new(1.0, 1.0), Complex::new(1.0, 1.0)]
    );
}
