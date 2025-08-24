use linear_algebra_42::Matrix;

#[test]
fn test_determinant_cases() {
    let cases = [
        ([[0.0, 0.0], [0.0, 0.0]], 0.0),
        ([[1.0, 0.0], [0.0, 1.0]], 1.0),
        ([[2.0, 0.0], [0.0, 2.0]], 4.0),
        ([[1.0, 1.0], [1.0, 1.0]], 0.0),
        ([[0.0, 1.0], [1.0, 0.0]], -1.0),
        ([[1.0, 2.0], [3.0, 4.0]], -2.0),
        ([[-7.0, 5.0], [4.0, 6.0]], -62.0),
    ];

    for (input, expected) in cases {
        let matrix = Matrix::from(input);
        let det: f32 = matrix.determinant();
        assert!((det - expected).abs() < 1e-4);
    }
}

#[test]
fn test_determinant_cases_2() {
    let cases = [([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], 1.0)];

    for (input, expected) in cases {
        let matrix = Matrix::from(input);
        let det: f32 = matrix.determinant();
        assert!((det - expected).abs() < 1e-4);
    }
}

#[test]
fn test_determinant_cases_3() {
    let cases = [(
        [
            [8.0, 5.0, -2.0, 4.0],
            [4.0, 2.5, 20.0, 4.0],
            [8.0, 5.0, 1.0, 4.0],
            [28.0, -4.0, 17.0, 1.0],
        ],
        1032.0,
    )];

    for (input, expected) in cases {
        let matrix = Matrix::from(input);
        let det: f32 = matrix.determinant();
        assert!((det - expected).abs() < 1e-4);
    }
}
