use linear_algebra_42::Vector;

#[test]
fn test_vector_add_cases() {
    let cases = [
        ([0., 0.], [0., 0.], [0., 0.]),
        ([1., 0.], [0., 1.], [1., 1.]),
        ([1., 1.], [1., 1.], [2., 2.]),
        ([21., 21.], [21., 21.], [42., 42.]),
        ([-21., 21.], [21., -21.], [0., 0.]),
    ];

    for (u, v, expected) in cases {
        let mut vec_u = Vector::from(u);
        let vec_v = Vector::from(v);
        vec_u.add_inline(&vec_v);
        assert_eq!(vec_u.to_vec(), expected);
    }
}

#[test]
fn test_vector_add_cases_2() {
    let cases = [(
        [0., 1., 2., 3., 4., 5., 6., 7., 8., 9.],
        [9., 8., 7., 6., 5., 4., 3., 2., 1., 0.],
        [9., 9., 9., 9., 9., 9., 9., 9., 9., 9.],
    )];

    for (u, v, expected) in cases {
        let mut vec_u = Vector::from(u);
        let vec_v = Vector::from(v);
        vec_u.add_inline(&vec_v);
        assert_eq!(vec_u.to_vec(), expected);
    }
}

#[test]
fn test_vector_sub_cases() {
    let cases = [
        ([0., 0.], [0., 0.], [0., 0.]),
        ([1., 0.], [0., 1.], [1., -1.]),
        ([1., 1.], [1., 1.], [0., 0.]),
        ([21., 21.], [21., 21.], [0., 0.]),
        ([-21., 21.], [21., -21.], [-42., 42.]),
    ];

    for (u, v, expected) in cases {
        let mut vec_u = Vector::from(u);
        let vec_v = Vector::from(v);
        vec_u.sub(&vec_v);
        assert_eq!(vec_u.to_vec(), expected);
    }
}

#[test]
fn test_vector_scale_cases() {
    let cases = [
        ([0., 0.], 1.0, [0., 0.]),
        ([1., 0.], 1.0, [1., 0.]),
        ([1., 1.], 2.0, [2., 2.]),
        ([21., 21.], 2.0, [42., 42.]),
        ([42., 42.], 0.5, [21., 21.]),
    ];

    for (u, scalar, expected) in cases {
        let mut vec_u = Vector::from(u);
        vec_u.scl(scalar);
        assert_eq!(vec_u.to_vec(), expected);
    }
}
