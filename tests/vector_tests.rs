use linear_algebra::{errors::VectorError, vector::Vector};

#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = [1, 2, 3, 4, 5];
        let vector = Vector::new(&data);

        assert_eq!(vector.len(), 5);
        assert_eq!(vector[0], 1);
        assert_eq!(vector[4], 5);
    }

    #[test]
    fn test_with_default() {
        let vector: Vector<i32> = Vector::with_default(3);

        assert_eq!(vector.len(), 3);
        assert_eq!(vector[0], 0);
        assert_eq!(vector[1], 0);
        assert_eq!(vector[2], 0);
    }

    #[test]
    fn test_len() {
        let data = [1.0, 2.0, 3.0];
        let vector = Vector::new(&data);

        assert_eq!(vector.len(), 3);
    }

    #[test]
    fn test_iter() {
        let data = [1, 2, 3];
        let vector = Vector::new(&data);

        let collected: Vec<&i32> = vector.iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);
    }

    #[test]
    fn test_add_success() {
        let mut vector1 = Vector::new(&[1, 2, 3]);
        let vector2 = Vector::new(&[4, 5, 6]);

        let result = vector1.add(&vector2);

        assert!(result.is_ok());
        assert_eq!(vector1[0], 5);
        assert_eq!(vector1[1], 7);
        assert_eq!(vector1[2], 9);
    }

    #[test]
    fn test_add_dimension_mismatch() {
        let mut vector1 = Vector::new(&[1, 2, 3]);
        let vector2 = Vector::new(&[4, 5]);

        let result = vector1.add(&vector2);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            VectorError::DimensionMismatch
        ));
    }

    #[test]
    fn test_sub_success() {
        let mut vector1 = Vector::new(&[10, 8, 6]);
        let vector2 = Vector::new(&[4, 3, 2]);

        let result = vector1.sub(&vector2);

        assert!(result.is_ok());
        assert_eq!(vector1[0], 6);
        assert_eq!(vector1[1], 5);
        assert_eq!(vector1[2], 4);
    }

    #[test]
    fn test_sub_dimension_mismatch() {
        let mut vector1 = Vector::new(&[1, 2, 3]);
        let vector2 = Vector::new(&[4, 5]);

        let result = vector1.sub(&vector2);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            VectorError::DimensionMismatch
        ));
    }

    #[test]
    fn test_scalar() {
        let mut vector = Vector::new(&[2, 3, 4]);

        vector.scalar(3);

        assert_eq!(vector[0], 6);
        assert_eq!(vector[1], 9);
        assert_eq!(vector[2], 12);
    }

    #[test]
    fn test_add_new_success() {
        let vector1 = Vector::new(&[1, 2, 3]);
        let vector2 = Vector::new(&[4, 5, 6]);

        let result = vector1.add_new(&vector2);

        assert!(result.is_ok());
        let new_vector = result.unwrap();
        assert_eq!(new_vector[0], 5);
        assert_eq!(new_vector[1], 7);
        assert_eq!(new_vector[2], 9);

        assert_eq!(vector1[0], 1);
        assert_eq!(vector2[0], 4);
    }

    #[test]
    fn test_add_new_dimension_mismatch() {
        let vector1 = Vector::new(&[1, 2, 3]);
        let vector2 = Vector::new(&[4, 5]);

        let result = vector1.add_new(&vector2);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            VectorError::DimensionMismatch
        ));
    }

    #[test]
    fn test_sub_new_success() {
        let vector1 = Vector::new(&[10, 8, 6]);
        let vector2 = Vector::new(&[4, 3, 2]);

        let result = vector1.sub_new(&vector2);

        assert!(result.is_ok());
        let new_vector = result.unwrap();
        assert_eq!(new_vector[0], 6);
        assert_eq!(new_vector[1], 5);
        assert_eq!(new_vector[2], 4);

        assert_eq!(vector1[0], 10);
        assert_eq!(vector2[0], 4);
    }

    #[test]
    fn test_sub_new_dimension_mismatch() {
        let vector1 = Vector::new(&[1, 2, 3]);
        let vector2 = Vector::new(&[4, 5]);

        let result = vector1.sub_new(&vector2);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            VectorError::DimensionMismatch
        ));
    }

    #[test]
    fn test_scalar_new() {
        let vector = Vector::new(&[2, 3, 4]);

        let new_vector = vector.scalar_new(3);

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
        let vector = Vector::new(&data);

        assert_eq!(vector[0], 1);
        assert_eq!(vector[2], 3);
        assert_eq!(vector[4], 5);
    }

    #[test]
    fn test_index_mut() {
        let data = [1, 2, 3];
        let mut vector = Vector::new(&data);

        vector[1] = 10;

        assert_eq!(vector[0], 1);
        assert_eq!(vector[1], 10);
        assert_eq!(vector[2], 3);
    }

    #[test]
    fn test_clone() {
        let vector1 = Vector::new(&[1, 2, 3]);
        let vector2 = vector1.clone();

        assert_eq!(vector1, vector2);
        assert_eq!(vector1[0], vector2[0]);
        assert_eq!(vector1[1], vector2[1]);
        assert_eq!(vector1[2], vector2[2]);
    }

    #[test]
    fn test_partial_eq() {
        let vector1 = Vector::new(&[1, 2, 3]);
        let vector2 = Vector::new(&[1, 2, 3]);
        let vector3 = Vector::new(&[1, 2, 4]);

        assert_eq!(vector1, vector2);
        assert_ne!(vector1, vector3);
    }

    #[test]
    fn test_with_floats() {
        let vector1 = Vector::new(&[1.5, 2.5, 3.5]);
        let vector2 = Vector::new(&[0.5, 1.5, 2.5]);

        let result = vector1.add_new(&vector2).unwrap();

        assert_eq!(result[0], 2.0);
        assert_eq!(result[1], 4.0);
        assert_eq!(result[2], 6.0);
    }
}
