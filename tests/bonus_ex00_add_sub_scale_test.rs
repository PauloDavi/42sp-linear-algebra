use linear_algebra_42::{linear_combination, Complex, Vector};

#[test]
fn test_vector_complex_add_and_dot() {
    let a = Vector::from([Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)]);

    let b = Vector::from([Complex::new(4.0, 1.0), Complex::new(-2.0, 2.0)]);

    let mut sum = a.clone();
    sum.add_inline(&b);

    assert_eq!(
        sum.to_vec(),
        [Complex::new(5.0, 3.0), Complex::new(1.0, 1.0),]
    );

    let dot = a.dot(&b);
    assert_eq!(dot, Complex::new(-2.0, 17.0));
}

#[test]
fn test_vector_complex_linear_combination() {
    let v1 = Vector::from([Complex::new(1.0, 0.0), Complex::new(0.0, 1.0)]);
    let v2 = Vector::from([Complex::new(0.0, 1.0), Complex::new(1.0, 0.0)]);

    let result =
        linear_combination([v1, v2], [Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)]).unwrap();

    assert_eq!(
        result.to_vec(),
        [Complex::new(1.0, 1.0), Complex::new(1.0, 1.0)]
    );
}

#[test]
fn test_vector_add_sub_scale_complex() {
    let mut v = Vector::from([Complex::new(1.0, 2.0), Complex::new(-3.0, 1.0)]);

    let w = Vector::from([Complex::new(2.0, -1.0), Complex::new(0.0, 2.0)]);

    v.add_inline(&w);
    assert_eq!(
        v.to_vec(),
        [Complex::new(3.0, 1.0), Complex::new(-3.0, 3.0)]
    );

    v.sub(&w);
    assert_eq!(
        v.to_vec(),
        [Complex::new(1.0, 2.0), Complex::new(-3.0, 1.0)]
    );

    v.scl(Complex::new(0.0, 1.0)); // multiplica por i
    assert_eq!(
        v.to_vec(),
        [Complex::new(-2.0, 1.0), Complex::new(-1.0, -3.0)]
    );
}
