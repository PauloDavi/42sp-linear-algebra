use linear_algebra_42::{linear_combination, Vector};

#[test]
fn test_linear_combination_case1() {
    let vectors = [Vector::from([-42.0, 42.0])];
    let coefs = [-1.0];
    let expected = [42.0, -42.0];

    let result = linear_combination(vectors, coefs).unwrap();
    assert_eq!(result.to_vec(), expected);
}

#[test]
fn test_linear_combination_case2() {
    let vectors = [
        Vector::from([-42.0]),
        Vector::from([-42.0]),
        Vector::from([-42.0]),
    ];
    let coefs = [-1.0, 1.0, 1.0];
    let expected = [-42.0];

    let result = linear_combination(vectors, coefs).unwrap();
    assert_eq!(result.to_vec(), expected);
}

#[test]
fn test_linear_combination_case3() {
    let vectors = [
        Vector::from([-42.0, 42.0]),
        Vector::from([1.0, 3.0]),
        Vector::from([10.0, 20.0]),
    ];
    let coefs = [1.0, -10.0, -1.0];
    let expected = [-62.0, -8.0];

    let result = linear_combination(vectors, coefs).unwrap();
    assert_eq!(result.to_vec(), expected);
}

#[test]
fn test_linear_combination_case4() {
    let vectors = [
        Vector::from([-42.0, 100.0, -69.5]),
        Vector::from([1.0, 3.0, 5.0]),
    ];
    let coefs = [1.0, -10.0];
    let expected = [-52.0, 70.0, -119.5];

    let result = linear_combination(vectors, coefs).unwrap();
    assert_eq!(result.to_vec(), expected);
}
