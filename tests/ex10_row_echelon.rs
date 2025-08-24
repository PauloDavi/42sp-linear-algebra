use linear_algebra_42::Matrix;

#[test]
fn test_row_echelon_cases() {
    let cases = [
        ([[0.0, 0.0], [0.0, 0.0]], [[0.0, 0.0], [0.0, 0.0]]),
        ([[1.0, 0.0], [0.0, 1.0]], [[1.0, 0.0], [0.0, 1.0]]),
        ([[4.0, 2.0], [2.0, 1.0]], [[1.0, 0.5], [0.0, 0.0]]),
        ([[-7.0, 2.0], [4.0, 8.0]], [[1.0, 0.0], [0.0, 1.0]]),
        ([[1.0, 2.0], [4.0, 8.0]], [[1.0, 2.0], [0.0, 0.0]]),
    ];

    for (input, expected) in cases {
        let matrix = Matrix::from(input);
        let echelon = matrix.row_echelon();
        for (row_e, row_r) in expected.iter().zip(echelon.iter()) {
            for (val_e, val_r) in row_e.iter().zip(row_r.iter()) {
                let result: f32 = val_e - val_r;
                assert!(result.abs() < 1e-4);
            }
        }
    }
}
