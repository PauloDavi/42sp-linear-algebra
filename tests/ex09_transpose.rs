use linear_algebra_42::Matrix;

#[test]
fn test_transpose_cases() {
    let cases = [
        ([[0.0, 0.0], [0.0, 0.0]], [[0.0, 0.0], [0.0, 0.0]]),
        ([[1.0, 0.0], [0.0, 1.0]], [[1.0, 0.0], [0.0, 1.0]]),
        ([[1.0, 2.0], [3.0, 4.0]], [[1.0, 3.0], [2.0, 4.0]]),
    ];

    for (input, expected) in cases {
        let matrix = Matrix::from(input);
        let transposed = matrix.transpose();
        assert_eq!(transposed, Matrix::from(expected));
    }
}

#[test]
fn test_transpose_cases_2() {
    let cases = [(
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    )];

    for (input, expected) in cases {
        let matrix = Matrix::from(input);
        let transposed = matrix.transpose();
        assert_eq!(transposed, Matrix::from(expected));
    }
}

#[test]
fn test_transpose_cases_3() {
    let cases = [(
        [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]],
        [[1.0, 3.0, 5.0], [2.0, 4.0, 6.0]],
    )];

    for (input, expected) in cases {
        let matrix = Matrix::from(input);
        let transposed = matrix.transpose();
        assert_eq!(transposed, Matrix::from(expected));
    }
}
