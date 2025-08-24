use linear_algebra_42::Matrix;

#[test]
fn test_trace_cases() {
    let cases = [
        ([[0.0, 0.0], [0.0, 0.0]], 0.0),
        ([[1.0, 0.0], [0.0, 1.0]], 2.0),
        ([[1.0, 2.0], [3.0, 4.0]], 5.0),
        ([[8.0, -7.0], [4.0, 2.0]], 10.0),
    ];

    for (data, expected) in cases {
        let matrix = Matrix::from(data);
        let result = matrix.trace();
        assert_eq!(result, expected);
    }
}

#[test]
fn test_trace_cases_2() {
    let cases = [([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], 3.0)];

    for (data, expected) in cases {
        let matrix = Matrix::from(data);
        let result = matrix.trace();
        assert_eq!(result, expected);
    }
}
