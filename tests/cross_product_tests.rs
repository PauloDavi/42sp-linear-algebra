#[cfg(test)]
mod cross_product_tests {
    use linear_algebra_42::{cross_product, vector::Vector};

    #[test]
    fn test_cross_product_basic() {
        let u = Vector::from([1, 0, 0]);
        let v = Vector::from([0, 1, 0]);

        let result = cross_product(&u, &v);

        assert_eq!(result[0], 0);
        assert_eq!(result[1], 0);
        assert_eq!(result[2], 1);
    }

    #[test]
    fn test_cross_product_reverse() {
        let u = Vector::from([0, 1, 0]);
        let v = Vector::from([1, 0, 0]);

        let result = cross_product(&u, &v);

        assert_eq!(result[0], 0);
        assert_eq!(result[1], 0);
        assert_eq!(result[2], -1);
    }

    #[test]
    fn test_cross_product_same_vectors() {
        let u = Vector::from([1, 2, 3]);
        let v = Vector::from([1, 2, 3]);

        let result = cross_product(&u, &v);

        assert_eq!(result[0], 0);
        assert_eq!(result[1], 0);
        assert_eq!(result[2], 0);
    }

    #[test]
    fn test_cross_product_general_case() {
        let u = Vector::from([1, 2, 3]);
        let v = Vector::from([4, 5, 6]);

        let result = cross_product(&u, &v);

        assert_eq!(result[0], -3);
        assert_eq!(result[1], 6);
        assert_eq!(result[2], -3);
    }

    #[test]
    fn test_cross_product_floats() {
        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([2.0, 3.0, 4.0]);

        let result = cross_product(&u, &v);

        assert_eq!(result[0], -1.0);
        assert_eq!(result[1], 2.0);
        assert_eq!(result[2], -1.0);
    }

    #[test]
    fn test_cross_product_invalid_dimensions() {
        let u = Vector::from([1, 2]);
        let v = Vector::from([3, 4, 5]);

        let result = cross_product(&u, &v);

        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_cross_product_4d_vectors() {
        let u = Vector::from([1, 2, 3, 4]);
        let v = Vector::from([5, 6, 7, 8]);

        let result = cross_product(&u, &v);

        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 3);
        assert_eq!(result[3], 4);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_cross_product_orthogonality() {
        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([4.0, 5.0, 6.0]);

        let result = cross_product(&u, &v);

        assert_eq!(u.dot(&result), 0.0);
        assert_eq!(v.dot(&result), 0.0);
    }

    #[test]
    fn test_cross_product_anticommutative() {
        let u = Vector::from([1, 2, 3]);
        let v = Vector::from([4, 5, 6]);

        let result1 = cross_product(&u, &v);
        let result2 = cross_product(&v, &u);

        assert_eq!(result1[0], -result2[0]);
        assert_eq!(result1[1], -result2[1]);
        assert_eq!(result1[2], -result2[2]);
    }
}
