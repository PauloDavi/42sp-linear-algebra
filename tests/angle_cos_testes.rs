#[cfg(test)]
mod angle_cos_tests {
    use std::f32::consts::FRAC_1_SQRT_2;

    use linear_algebra_42::{angle_cos, vector::Vector};

    #[test]
    fn test_angle_cos_same_vectors() {
        let u = Vector::from([1.0, 0.0]);
        let v = Vector::from([1.0, 0.0]);

        let result = angle_cos(&u, &v);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_angle_cos_orthogonal_vectors() {
        let u = Vector::from([1.0, 0.0]);
        let v = Vector::from([0.0, 1.0]);

        let result = angle_cos(&u, &v);

        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_angle_cos_opposite_vectors() {
        let u = Vector::from([-1.0, 1.0]);
        let v = Vector::from([1.0, -1.0]);

        let result = angle_cos(&u, &v);

        assert!((result - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_angle_cos_parallel_vectors() {
        let u = Vector::from([2.0, 1.0]);
        let v = Vector::from([4.0, 2.0]);

        let result = angle_cos(&u, &v);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_angle_cos_3d_vectors() {
        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([4.0, 5.0, 6.0]);

        let result = angle_cos(&u, &v);

        let expected = 32.0 / ((14.0_f32).sqrt() * (77.0_f32).sqrt());
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_angle_cos_unit_vectors() {
        let u = Vector::from([1.0, 0.0]);
        let v = Vector::from([FRAC_1_SQRT_2, FRAC_1_SQRT_2]);

        let result = angle_cos(&u, &v);

        let expected = FRAC_1_SQRT_2;
        assert!((result - expected).abs() < 1e-6);
    }
}
