use linear_algebra_42::Matrix;

#[test]
fn test_inverse_cases() {
    let cases = [
        ([[1.0, 0.0], [0.0, 1.0]], Some([[1.0, 0.0], [0.0, 1.0]])),
        ([[2.0, 0.0], [0.0, 2.0]], Some([[0.5, 0.0], [0.0, 0.5]])),
        ([[0.5, 0.0], [0.0, 0.5]], Some([[2.0, 0.0], [0.0, 2.0]])),
        ([[1.0, 2.0], [3.0, 4.0]], Some([[-2.0, 1.0], [1.5, -0.5]])),
    ];

    for (input, expected_opt) in cases {
        let m = Matrix::from(input);
        let result = m.inverse();

        match expected_opt {
            Some(expected) => {
                assert!(result.is_ok());
                let result_matrix = result.unwrap();
                for (row_e, row_r) in expected.iter().zip(result_matrix.iter()) {
                    for (val_e, val_r) in row_e.iter().zip(row_r.iter()) {
                        let result: f32 = val_e - val_r;
                        assert!(result.abs() < 1e-4);
                    }
                }
            }
            None => {
                assert!(result.is_err());
            }
        }
    }
}

#[test]
fn test_inverse_cases_2() {
    let cases = [(
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        Some([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]),
    )];

    for (input, expected_opt) in cases {
        let m = Matrix::from(input);
        let result = m.inverse();

        match expected_opt {
            Some(expected) => {
                assert!(result.is_ok());
                let result_matrix = result.unwrap();
                for (row_e, row_r) in expected.iter().zip(result_matrix.iter()) {
                    for (val_e, val_r) in row_e.iter().zip(row_r.iter()) {
                        let result: f32 = val_e - val_r;
                        assert!(result.abs() < 1e-4);
                    }
                }
            }
            None => {
                assert!(result.is_err());
            }
        }
    }
}

#[test]
fn test_inverse_cases_3() {
    let m = Matrix::from([[1.0f32, 2.0], [2.0, 4.0]]);
    let result = m.inverse();
    assert!(result.is_err());
}
