use linear_algebra_42::{angle_cos, Vector};

#[test]
fn test_angle_cos_official_cases() {
    let cases = vec![
        (vec![1.0, 0.0], vec![0.0, 1.0], 0.0),
        (vec![8.0, 7.0], vec![3.0, 2.0], 0.9914542955425437),
        (vec![1.0, 1.0], vec![1.0, 1.0], 1.0),
        (vec![4.0, 2.0], vec![1.0, 1.0], 0.9486832980505138),
        (vec![-7.0, 3.0], vec![6.0, 4.0], -0.5462677805469223),
    ];

    for (u_data, v_data, expected) in cases {
        let u = Vector::from(u_data);
        let v = Vector::from(v_data);
        let result = angle_cos(&u, &v);
        assert!((result - expected).abs() < 1e-6);
    }
}
