use linear_algebra_42::{Complex, Matrix};

#[test]
fn test_trace_complex_matrix() {
    let m = Matrix::from([
        [Complex::new(1.0, 2.0), Complex::new(0.0, 1.0)],
        [Complex::new(5.0, -1.0), Complex::new(2.0, -2.0)],
    ]);

    let trace = m.trace();

    assert_eq!(trace, Complex::new(3.0, 0.0)); // (1+2i) + (2-2i) = 3 + 0i
}
