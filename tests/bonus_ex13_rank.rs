use linear_algebra_42::{Complex, Matrix};

#[test]
fn test_rank_complex_matrix() {
    let mut m = Matrix::from([
        [Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)],
        [Complex::new(2.0, 0.0), Complex::new(4.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
    ]);

    let rank = m.rank();
    assert_eq!(rank, 1);
}
