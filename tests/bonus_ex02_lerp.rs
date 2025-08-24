use linear_algebra_42::{lerp, Complex, Vector};

#[test]
fn test_lerp_halfway() {
    let u = Vector::from([Complex::new(0.0, 0.0), Complex::new(2.0, 2.0)]);
    let v = Vector::from([Complex::new(2.0, 2.0), Complex::new(0.0, 0.0)]);

    let result = lerp(u.clone(), v.clone(), 0.5).unwrap();

    assert_eq!(
        result,
        Vector::from([Complex::new(1.0, 1.0), Complex::new(1.0, 1.0)])
    );
}

#[test]
fn test_lerp_edge_cases() {
    let u = Vector::from([Complex::new(-1.0, 1.0), Complex::new(0.0, -2.0)]);
    let v = Vector::from([Complex::new(1.0, -1.0), Complex::new(2.0, 0.0)]);

    let result_0 = lerp(u.clone(), v.clone(), 0.0).unwrap();
    let result_1 = lerp(u.clone(), v.clone(), 1.0).unwrap();

    assert_eq!(result_0, u);
    assert_eq!(result_1, v);
}
