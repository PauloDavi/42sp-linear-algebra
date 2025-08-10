#[cfg(test)]
mod complex_matrix_tests {
    use linear_algebra::{Complex, Matrix, Vector};

    #[test]
    fn test_conjugate_transpose_real_matrix() {
        let m = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let expected = Matrix::from([[1.0, 3.0], [2.0, 4.0]]);

        assert_eq!(m.conjugate_transpose(), expected);
        assert_eq!(m.conjugate_transpose(), m.transpose());
    }

    #[test]
    fn test_conjugate_transpose_complex_matrix() {
        let m = Matrix::from([
            [Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)],
            [Complex::new(0.0, 1.0), Complex::new(2.0, 0.0)],
        ]);

        let expected = Matrix::from([
            [Complex::new(1.0, -2.0), Complex::new(0.0, -1.0)],
            [Complex::new(3.0, 1.0), Complex::new(2.0, 0.0)],
        ]);

        assert_eq!(m.conjugate_transpose(), expected);
    }

    #[test]
    fn test_conjugate_transpose_identity() {
        let identity = Matrix::from([
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        ]);

        assert_eq!(identity.conjugate_transpose(), identity);
    }

    #[test]
    fn test_conjugate_transpose_hermitian_matrix() {
        let hermitian = Matrix::from([
            [Complex::new(1.0, 0.0), Complex::new(1.0, 2.0)],
            [Complex::new(1.0, -2.0), Complex::new(3.0, 0.0)],
        ]);

        assert_eq!(hermitian.conjugate_transpose(), hermitian);
    }

    #[test]
    fn test_complex_dot_product_real_vectors() {
        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([4.0, 5.0, 6.0]);

        assert_eq!(u.complex_dot(&v), u.dot(&v));
        assert_eq!(u.complex_dot(&v), 32.0);
    }

    #[test]
    fn test_complex_dot_product_complex_vectors() {
        let u = Vector::from([Complex::new(1.0, 1.0), Complex::new(2.0, -1.0)]);
        let v = Vector::from([Complex::new(1.0, 1.0), Complex::new(0.0, 1.0)]);

        let expected = Complex::new(1.0, 2.0);
        assert_eq!(u.complex_dot(&v), expected);
    }

    #[test]
    fn test_complex_dot_product_orthogonal_vectors() {
        let u = Vector::from([Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]);
        let v = Vector::from([Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]);

        let result = u.complex_dot(&v);
        assert_eq!(result, Complex::new(0.0, 0.0));
    }

    #[test]
    fn test_complex_dot_product_self() {
        let u = Vector::from([Complex::new(3.0, 4.0), Complex::new(0.0, 5.0)]);

        let result = u.complex_dot(&u);
        assert_eq!(result, Complex::new(50.0, 0.0));
        assert_eq!(result.imaginary(), 0.0);
    }

    #[test]
    fn test_conjugate_transpose_double_application() {
        let m = Matrix::from([
            [Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)],
            [Complex::new(0.0, 1.0), Complex::new(2.0, 0.0)],
        ]);

        assert_eq!(m.conjugate_transpose().conjugate_transpose(), m);
    }

    #[test]
    fn test_sesquilinear_property() {
        let u = Vector::from([Complex::new(1.0, 1.0), Complex::new(2.0, -1.0)]);
        let v = Vector::from([Complex::new(1.0, 1.0), Complex::new(0.0, 1.0)]);

        let uv = u.complex_dot(&v);
        let vu = v.complex_dot(&u);

        assert_eq!(vu, uv.conjugate());
    }
}
