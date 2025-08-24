use linear_algebra_42::{cross_product, Complex, Vector};

#[test]
fn test_cross_product_cases_complex() {
    let cases = [
        (
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
        ),
        (
            [
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
        ),
        (
            [
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0),
            ],
        ),
        (
            [
                Complex::new(8.0, 0.0),
                Complex::new(7.0, 0.0),
                Complex::new(-4.0, 0.0),
            ],
            [
                Complex::new(3.0, 0.0),
                Complex::new(2.0, 0.0),
                Complex::new(1.0, 0.0),
            ],
            [
                Complex::new(15.0, 0.0),
                Complex::new(-20.0, 0.0),
                Complex::new(-5.0, 0.0),
            ],
        ),
        (
            [
                Complex::new(1.0, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(1.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
        ),
        (
            [
                Complex::new(1.0, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(1.0, 0.0),
            ],
            [
                Complex::new(1.0, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(1.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
        ),
        // Casos com parte imaginária
        (
            [
                Complex::new(1.0, 2.0),
                Complex::new(0.0, 1.0),
                Complex::new(0.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 1.0),
            ],
            [
                Complex::new(1.0, 1.0),
                Complex::new(-2.0, 0.0),
                Complex::new(1.0, -1.0),
            ],
        ),
        (
            [
                Complex::new(0.0, 1.0),
                Complex::new(2.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 1.0),
                Complex::new(1.0, 0.0),
            ],
            [
                Complex::new(0.0, -2.0),
                Complex::new(0.0, 0.0),
                Complex::new(2.0, -1.0),
            ],
        ),
        (
            [
                Complex::new(1.0, 1.0),
                Complex::new(1.0, 1.0),
                Complex::new(1.0, 1.0),
            ],
            [
                Complex::new(1.0, -1.0),
                Complex::new(1.0, -1.0),
                Complex::new(1.0, -1.0),
            ],
            [
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
        ),
    ];

    for (u_data, v_data, expected) in cases {
        let u = Vector::from(u_data);
        let v = Vector::from(v_data);
        let result = cross_product(&u, &v);
        assert_eq!(result.to_vec(), expected);
    }
}
