use linear_algebra_42::Matrix;

#[test]
fn test_rank_cases() {
    let cases = [
        ([[0.0, 0.0], [0.0, 0.0]], 0),
        ([[1.0, 0.0], [0.0, 1.0]], 2),
        ([[2.0, 0.0], [0.0, 2.0]], 2),
        ([[1.0, 1.0], [1.0, 1.0]], 1),
        ([[0.0, 1.0], [1.0, 0.0]], 2),
        ([[1.0, 2.0], [3.0, 4.0]], 2),
        ([[-7.0, 5.0], [4.0, 6.0]], 2),
    ];

    for (data, expected) in cases {
        let mut matrix = Matrix::from(data);
        let result = matrix.rank();
        assert_eq!(result, expected);
    }
}

#[test]
fn test_rank_cases_2() {
    let cases = [([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], 3)];

    for (data, expected) in cases {
        let mut matrix = Matrix::from(data);
        let result = matrix.rank();
        assert_eq!(result, expected);
    }
}
