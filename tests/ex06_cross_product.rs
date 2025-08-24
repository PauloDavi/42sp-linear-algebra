use linear_algebra_42::{cross_product, Vector};

#[test]
fn test_cross_product_cases() {
    let cases = vec![
        (
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ),
        (
            vec![1.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ),
        (
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ),
        (
            vec![8.0, 7.0, -4.0],
            vec![3.0, 2.0, 1.0],
            vec![15.0, -20.0, -5.0],
        ),
        (
            vec![1.0, 1.0, 1.0],
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ),
        (
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
            vec![0.0, 0.0, 0.0],
        ),
    ];

    for (u_data, v_data, expected) in cases {
        let u = Vector::from(u_data);
        let v = Vector::from(v_data);
        let result = cross_product(&u, &v);
        assert_eq!(result.to_vec(), expected);
    }
}
