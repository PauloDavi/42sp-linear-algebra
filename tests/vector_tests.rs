#[cfg(test)]
mod vector_tests {
    use linear_algebra_42::vector::Vector;

    #[test]
    fn test_new() {
        let data = [1, 2, 3, 4, 5];
        let vector = Vector::from(data);

        assert_eq!(vector.len(), 5);
        assert_eq!(vector[0], 1);
        assert_eq!(vector[4], 5);
    }

    #[test]
    fn test_zeros() {
        let vector: Vector<i32> = Vector::zeros(3);

        assert_eq!(vector.len(), 3);
        assert_eq!(vector[0], 0);
        assert_eq!(vector[1], 0);
        assert_eq!(vector[2], 0);
    }

    #[test]
    fn test_len() {
        let data = [1.0, 2.0, 3.0];
        let vector = Vector::from(data);

        assert_eq!(vector.len(), 3);
    }

    #[test]
    fn test_iter() {
        let data = [1, 2, 3];
        let vector = Vector::from(data);

        let collected: Vec<&i32> = vector.iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);
    }

    #[test]
    fn test_add_success() {
        let mut vector1 = Vector::from([1, 2, 3]);
        let vector2 = Vector::from([4, 5, 6]);

        vector1.add_inline(&vector2);

        assert_eq!(vector1[0], 5);
        assert_eq!(vector1[1], 7);
        assert_eq!(vector1[2], 9);
    }

    #[test]
    fn test_sub_success() {
        let mut vector1 = Vector::from([10, 8, 6]);
        let vector2 = Vector::from([4, 3, 2]);

        vector1.sub(&vector2);

        assert_eq!(vector1[0], 6);
        assert_eq!(vector1[1], 5);
        assert_eq!(vector1[2], 4);
    }

    #[test]
    fn test_scl() {
        let mut vector = Vector::from([2, 3, 4]);

        vector.scl(3);

        assert_eq!(vector[0], 6);
        assert_eq!(vector[1], 9);
        assert_eq!(vector[2], 12);
    }

    #[test]
    fn test_add_new_success() {
        let vector1 = Vector::from([1, 2, 3]);
        let vector2 = Vector::from([4, 5, 6]);

        let new_vector = vector1.add_new(&vector2);

        assert_eq!(new_vector[0], 5);
        assert_eq!(new_vector[1], 7);
        assert_eq!(new_vector[2], 9);

        assert_eq!(vector1[0], 1);
        assert_eq!(vector2[0], 4);
    }

    #[test]
    fn test_sub_new_success() {
        let vector1 = Vector::from([10, 8, 6]);
        let vector2 = Vector::from([4, 3, 2]);

        let new_vector = vector1.sub_new(&vector2);

        assert_eq!(new_vector[0], 6);
        assert_eq!(new_vector[1], 5);
        assert_eq!(new_vector[2], 4);

        assert_eq!(vector1[0], 10);
        assert_eq!(vector2[0], 4);
    }

    #[test]
    fn test_scl_new() {
        let vector = Vector::from([2, 3, 4]);

        let new_vector = vector.scl_new(3);

        assert_eq!(new_vector[0], 6);
        assert_eq!(new_vector[1], 9);
        assert_eq!(new_vector[2], 12);

        assert_eq!(vector[0], 2);
        assert_eq!(vector[1], 3);
        assert_eq!(vector[2], 4);
    }

    #[test]
    fn test_index() {
        let data = [1, 2, 3, 4, 5];
        let vector = Vector::from(data);

        assert_eq!(vector[0], 1);
        assert_eq!(vector[2], 3);
        assert_eq!(vector[4], 5);
    }

    #[test]
    fn test_index_mut() {
        let data = [1, 2, 3];
        let mut vector = Vector::from(data);

        vector[1] = 10;

        assert_eq!(vector[0], 1);
        assert_eq!(vector[1], 10);
        assert_eq!(vector[2], 3);
    }

    #[test]
    fn test_clone() {
        let vector1 = Vector::from([1, 2, 3]);
        let vector2 = vector1.clone();

        assert_eq!(vector1, vector2);
        assert_eq!(vector1[0], vector2[0]);
        assert_eq!(vector1[1], vector2[1]);
        assert_eq!(vector1[2], vector2[2]);
    }

    #[test]
    fn test_partial_eq() {
        let vector1 = Vector::from([1, 2, 3]);
        let vector2 = Vector::from([1, 2, 3]);
        let vector3 = Vector::from([1, 2, 4]);

        assert_eq!(vector1, vector2);
        assert_ne!(vector1, vector3);
    }

    #[test]
    fn test_with_floats() {
        let vector1 = Vector::from([1.5, 2.5, 3.5]);
        let vector2 = Vector::from([0.5, 1.5, 2.5]);

        let result = vector1.add_new(&vector2);

        assert_eq!(result[0], 2.0);
        assert_eq!(result[1], 4.0);
        assert_eq!(result[2], 6.0);
    }

    #[test]
    fn test_dot_product() {
        let vector1 = Vector::from([1, 2, 3]);
        let vector2 = Vector::from([4, 5, 6]);

        let result = vector1.dot(&vector2);

        assert_eq!(result, 32);
    }

    #[test]
    fn test_dot_product_floats() {
        let vector1 = Vector::from([1.0, 2.0, 3.0]);
        let vector2 = Vector::from([2.0, 3.0, 4.0]);

        let result = vector1.dot(&vector2);

        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_dot_product_zero() {
        let vector1 = Vector::from([1, 2, 3]);
        let vector2 = Vector::from([0, 0, 0]);

        let result = vector1.dot(&vector2);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_norm_1() {
        let vector = Vector::from([1i16, -2i16, 3i16]);

        let result = vector.norm_1();

        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_norm_1_floats() {
        let vector = Vector::from([1.5, -2.5, 3.0]);

        let result = vector.norm_1();

        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_norm() {
        let vector = Vector::from([3i16, 4i16]);

        let result = vector.norm();

        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_norm_floats() {
        let vector = Vector::from([1.0, 2.0, 2.0]);

        let result = vector.norm();

        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_norm_inf() {
        let vector = Vector::from([1i16, -5i16, 3i16]);

        let result = vector.norm_inf();

        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_norm_inf_floats() {
        let vector = Vector::from([1.5, -2.8, 2.3]);

        let result = vector.norm_inf();

        assert_eq!(result, 2.8);
    }
}
