use linear_algebra_42::{lerp, Vector};

#[test]
fn test_lerp_scalar_cases() {
    let cases = vec![
        (0.0, 1.0, 0.0, 0.0),
        (0.0, 1.0, 1.0, 1.0),
        (0.0, 42.0, 0.5, 21.0),
        (-42.0, 42.0, 0.5, 0.0),
    ];

    for (a, b, t, expected) in cases {
        let result = lerp(a, b, t).unwrap();
        assert_eq!(result, expected);
    }
}

#[test]
fn test_lerp_vector_case() {
    let u = Vector::from([-42.0, 42.0]);
    let v = Vector::from([42.0, -42.0]);
    let result = lerp(u, v, 0.5).unwrap();
    assert_eq!(result.to_vec(), vec![0.0, 0.0]);
}
