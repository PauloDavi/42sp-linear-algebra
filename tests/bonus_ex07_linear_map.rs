use linear_algebra_42::{Complex, Matrix, Vector};

#[test]
fn test_matrix_complex_add_and_sub() {
    let a = Matrix::from([
        [Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)],
        [Complex::new(-1.0, -1.0), Complex::new(0.0, 1.0)],
    ]);

    let b = Matrix::from([
        [Complex::new(0.0, 1.0), Complex::new(1.0, -1.0)],
        [Complex::new(2.0, 0.0), Complex::new(1.0, 1.0)],
    ]);

    let mut add_result = a.clone();
    add_result.add(&b);

    assert_eq!(
        add_result,
        Matrix::from([
            [Complex::new(1.0, 3.0), Complex::new(4.0, 3.0)],
            [Complex::new(1.0, -1.0), Complex::new(1.0, 2.0)],
        ])
    );

    let mut sub_result = a.clone();
    sub_result.sub(&b);

    assert_eq!(
        sub_result,
        Matrix::from([
            [Complex::new(1.0, 1.0), Complex::new(2.0, 5.0)],
            [Complex::new(-3.0, -1.0), Complex::new(-1.0, 0.0)],
        ])
    );
}

#[test]
fn test_matrix_complex_mul_mat() {
    let a = Matrix::from([
        [Complex::new(1.0, 1.0), Complex::new(2.0, 0.0)],
        [Complex::new(0.0, -1.0), Complex::new(1.0, 1.0)],
    ]);

    let b = Matrix::from([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 1.0)],
        [Complex::new(2.0, -1.0), Complex::new(1.0, 0.0)],
    ]);

    let result = a.mul_mat(&b);

    assert_eq!(
        result,
        Matrix::from([
            [Complex::new(5.0, -1.0), Complex::new(1.0, 1.0)],
            [Complex::new(3.0, 0.0), Complex::new(2.0, 1.0)],
        ])
    );
}

#[test]
fn test_matrix_complex_mul_vec() {
    let m = Matrix::from([
        [Complex::new(1.0, 1.0), Complex::new(0.0, -1.0)],
        [Complex::new(2.0, 0.0), Complex::new(1.0, 1.0)],
    ]);

    let v = Vector::from([Complex::new(1.0, 0.0), Complex::new(0.0, 1.0)]);

    let result = m.mul_vec(&v);

    assert_eq!(
        result,
        Vector::from([Complex::new(1.0, 3.0), Complex::new(-1.0, 0.0)])
    );
}
