use linear_algebra_42::Vector;

#[test]
fn test_vector_norms() {
    let cases = vec![
        (vec![0.0], 0.0, 0.0, 0.0),
        (vec![1.0], 1.0, 1.0, 1.0),
        (vec![0.0, 0.0], 0.0, 0.0, 0.0),
        (vec![1.0, 0.0], 1.0, 1.0, 1.0),
        (vec![2.0, 1.0], 3.0, 2.236067977, 2.0),
        (vec![4.0, 2.0], 6.0, 4.472135955, 4.0),
        (vec![-4.0, -2.0], 6.0, 4.472135955, 4.0),
    ];

    for (data, expected_1, expected_2, expected_inf) in cases {
        let v = Vector::from(data);
        assert!((v.norm_1() - expected_1).abs() < 1e-6);
        assert!((v.norm() - expected_2).abs() < 1e-6);
        assert!((v.norm_inf() - expected_inf).abs() < 1e-6);
    }
}
