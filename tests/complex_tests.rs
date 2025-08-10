use linear_algebra::{Complex, Magnitude, Negative, One, Zero};

#[cfg(test)]
mod complex_tests {
    use super::*;

    #[test]
    fn test_complex_creation() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.real(), 3.0);
        assert_eq!(c.imaginary(), 4.0);
    }

    #[test]
    fn test_complex_zero() {
        let zero = Complex::zero();
        assert_eq!(zero.real(), 0.0);
        assert_eq!(zero.imaginary(), 0.0);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_complex_one() {
        let one = Complex::one();
        assert_eq!(one.real(), 1.0);
        assert_eq!(one.imaginary(), 0.0);
    }

    #[test]
    fn test_complex_negative_one() {
        let neg_one = Complex::negative_one();
        assert_eq!(neg_one.real(), -1.0);
        assert_eq!(neg_one.imaginary(), 0.0);
    }

    #[test]
    fn test_complex_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.magnitude(), 5.0);
    }

    #[test]
    fn test_complex_addition() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        let result = c1 + c2;
        assert_eq!(result.real(), 4.0);
        assert_eq!(result.imaginary(), 6.0);
    }

    #[test]
    fn test_complex_subtraction() {
        let c1 = Complex::new(5.0, 7.0);
        let c2 = Complex::new(2.0, 3.0);
        let result = c1 - c2;
        assert_eq!(result.real(), 3.0);
        assert_eq!(result.imaginary(), 4.0);
    }

    #[test]
    fn test_complex_multiplication() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        let result = c1 * c2;
        assert_eq!(result.real(), -5.0);
        assert_eq!(result.imaginary(), 10.0);
    }

    #[test]
    fn test_complex_scalar_multiplication() {
        let c = Complex::new(2.0, 3.0);
        let result = c * 2.0;
        assert_eq!(result.real(), 4.0);
        assert_eq!(result.imaginary(), 6.0);
    }

    #[test]
    fn test_complex_division() {
        let c1 = Complex::new(1.0, 0.0);
        let c2 = Complex::new(1.0, 1.0);
        let result = c1 / c2;
        assert!((result.real() - 0.5).abs() < 1e-6);
        assert!((result.imaginary() - (-0.5)).abs() < 1e-6);
    }

    #[test]
    fn test_complex_negation() {
        let c = Complex::new(3.0, -4.0);
        let result = -c;
        assert_eq!(result.real(), -3.0);
        assert_eq!(result.imaginary(), 4.0);
    }

    #[test]
    fn test_complex_conjugate() {
        let c = Complex::new(3.0, 4.0);
        let conj = c.conjugate();
        assert_eq!(conj.real(), 3.0);
        assert_eq!(conj.imaginary(), -4.0);
    }

    #[test]
    fn test_complex_partial_ord() {
        let c1 = Complex::new(3.0, 4.0);
        let c2 = Complex::new(5.0, 0.0);
        let c3 = Complex::new(1.0, 1.0);

        assert_eq!(c1.partial_cmp(&c2), Some(std::cmp::Ordering::Equal));
        assert_eq!(c1.partial_cmp(&c3), Some(std::cmp::Ordering::Greater));
        assert_eq!(c3.partial_cmp(&c1), Some(std::cmp::Ordering::Less));
    }

    #[test]
    fn test_complex_clone_copy() {
        let c1 = Complex::new(2.0, 3.0);
        let c2 = c1;
        let c3 = c1;

        assert_eq!(c1, c2);
        assert_eq!(c1, c3);
        assert_eq!(c2, c3);
    }

    #[test]
    fn test_complex_equality() {
        let c1 = Complex::new(2.0, 3.0);
        let c2 = Complex::new(2.0, 3.0);
        let c3 = Complex::new(3.0, 2.0);

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }
}
