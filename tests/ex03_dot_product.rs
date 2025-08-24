use linear_algebra_42::Vector;

#[test]
fn test_dot_product_official_cases() {
    let cases = [
        ([0.0, 0.0], [0.0, 0.0], 0.0),
        ([1.0, 0.0], [0.0, 0.0], 0.0),
        ([1.0, 0.0], [1.0, 0.0], 1.0),
        ([1.0, 0.0], [0.0, 1.0], 0.0),
        ([1.0, 1.0], [1.0, 1.0], 2.0),
        ([4.0, 2.0], [2.0, 1.0], 10.0),
    ];

    for (u_data, v_data, expected) in cases {
        let u = Vector::from(u_data);
        let v = Vector::from(v_data);
        let result = u.dot(&v);
        assert_eq!(result, expected);
    }
}
