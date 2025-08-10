use linear_algebra::{Matrix, Vector, angle_cos, cross_product, lerp, linear_combination};

fn main() {
    println!("=== Vector Demonstration ===");
    let mut v1 = Vector::from([1, 2, 3]);
    let v2 = Vector::from([4, 5, 6]);

    println!("Vector 1: {v1}");
    println!("Vector 2: {v2}");

    v1.add_inline(&v2);
    println!("\nAfter addition: {v1}");

    v1.sub(&v2);
    println!("After subtraction: {v1}");

    v1.scl(3);
    println!("Vector 1 multiplied by 3: {v1}");

    println!("Addition returning a new: {}", v1.add_new(&v2));

    println!("Subtraction returning a new: {}", v1.sub_new(&v2));

    println!("Multiplied by 3 returning a new: {}", v1.scl_new(3));

    println!("\n=== Matrix Demonstration ===");
    let mut matrix1 = Matrix::from([[1, 2], [3, 4]]);
    let matrix2 = Matrix::from([[5, 6], [7, 8]]);

    println!("Matrix 1:\n{matrix1}");
    println!("\nMatrix 2:\n{matrix2}");

    matrix1.add(&matrix2);
    println!("\nAfter addition:\n{matrix1}");

    matrix1.sub(&matrix2);
    println!("\nAfter subtraction:\n{matrix1}");

    matrix1.scl(3);
    println!("\nMatrix 1 multiplied by 3:\n{matrix1}");

    println!("\nAddition returning a new:\n{}", matrix1.add_new(&matrix2));

    println!(
        "\nSubtraction returning a new:\n{}",
        matrix1.sub_new(&matrix2)
    );

    println!("\nMultiplied by 3 returning a new:\n{}", matrix1.scl_new(3));

    println!("\n=== Linear Combination Demonstration ===");
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    println!("e1: {e1}");
    println!("e2: {e2}");
    println!("e3: {e3}");

    let coefficients = [10., -2., 0.5];
    let result = linear_combination([e1, e2, e3], coefficients).unwrap();
    println!(
        "({} * e1) + ({} * e2) + ({} * e3) = {result}",
        coefficients[0], coefficients[1], coefficients[2]
    );

    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("\nv1: {v1}");
    println!("v2: {v2}");

    let coefficients = [10., -2.];
    let result = linear_combination([v1, v2], coefficients).unwrap();
    println!(
        "({} * v1) + ({} * v2) = {result}",
        coefficients[0], coefficients[1]
    );

    println!("\n=== Linear Interpolation Demonstration ===");
    println!("Applying lerp on scalars: {}", lerp(1.2, 1.4, 0.5).unwrap());

    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!(
        "\nApplying lerp on vectors:\n{}",
        lerp(v1, v2, 0.4).unwrap()
    );

    let matrix1 = Matrix::from([[1., 2.], [3., 4.]]);
    let matrix2 = Matrix::from([[5., 6.], [7., 8.]]);
    println!(
        "\nApplying lerp on matrices:\n{}",
        lerp(matrix1, matrix2, 0.3).unwrap()
    );

    println!("\n=== Dot Product Demonstration ===");

    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("Dot product between {u} and {v}: {}", u.dot(&v));

    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("Dot product between {u} and {v}: {}", u.dot(&v));

    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("Dot product between {u} and {v}: {}", u.dot(&v));

    println!("\n=== Vector Norms Demonstration ===");
    let u = Vector::from([0., 0., 0.]);
    println!(
        "For vector {u}: norm-1 = {}, norm-2 = {}, norm-infinity = {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );

    let u = Vector::from([1., 2., 3.]);
    println!(
        "For vector {u}: norm-1 = {}, norm-2 = {}, norm-infinity = {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );

    let u = Vector::from([-1., -2.]);
    println!(
        "For vector {u}: norm-1 = {}, norm-2 = {}, norm-infinity = {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );

    println!("\n=== Cosine of Angle Between Vectors Demonstration ===");
    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("Cosine of angle between {u} and {v}: {}", angle_cos(&u, &v));

    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("Cosine of angle between {u} and {v}: {}", angle_cos(&u, &v));

    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);
    println!("Cosine of angle between {u} and {v}: {}", angle_cos(&u, &v));

    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("Cosine of angle between {u} and {v}: {}", angle_cos(&u, &v));

    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("Cosine of angle between {u} and {v}: {}", angle_cos(&u, &v));

    println!("\n=== Cross Product Demonstration ===");
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!(
        "Cross product between {u} and {v}: {}",
        cross_product(&u, &v)
    );

    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!(
        "Cross product between {u} and {v}: {}",
        cross_product(&u, &v)
    );

    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!(
        "Cross product between {u} and {v}: {}",
        cross_product(&u, &v)
    );

    println!("\n=== Matrix x Vector Multiplication Demonstration ===");
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from([4., 2.]);
    println!("Multiplying identity matrix by {v}:\n{}", u.mul_vec(&v));

    let u = Matrix::from([[2., 0.], [0., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("\nMultiplying scaling matrix by {v}:\n{}", u.mul_vec(&v));

    let u = Matrix::from([[2., -2.], [-2., 2.]]);
    let v = Vector::from([4., 2.]);
    println!(
        "\nMultiplying matrix with negative elements by {v}:\n{}",
        u.mul_vec(&v)
    );

    println!("\n=== Matrix x Matrix Multiplication Demonstration ===");
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[1., 0.], [0., 1.]]);
    println!(
        "Multiplying identity matrix by identity matrix:\n{}",
        u.mul_mat(&v)
    );

    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!(
        "\nMultiplying identity matrix by another matrix:\n{}",
        u.mul_mat(&v)
    );

    let u = Matrix::from([[3., -5.], [6., 8.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("\nMultiplying generic matrices:\n{}", u.mul_mat(&v));

    println!("\n=== Matrix Trace Demonstration ===");
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("Trace of identity matrix:\n{u}\nTrace: {}", u.trace());

    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("\nTrace of 3x3 matrix:\n{u}\nTrace: {}", u.trace());

    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    println!("\nTrace of 3x3 matrix:\n{u}\nTrace: {}", u.trace());

    println!("\n=== Matrix Transposition Demonstration ===");
    let u = Matrix::from([[1., 1.], [0., 0.]]);
    println!("Original matrix:\n{u}\nTranspose:\n{}", u.transpose());

    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.]]);
    println!("\nOriginal matrix:\n{u}\nTranspose:\n{}", u.transpose());

    let u = Matrix::from([[-2., -8.], [1., -23.], [0., 6.]]);
    println!("\nOriginal matrix:\n{u}\nTranspose:\n{}", u.transpose());

    println!("\n=== Solving Systems with Matrices Demonstration ===");
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("\nOriginal matrix:\n{u}\nEchelon:\n{}", u.row_echelon());

    let u = Matrix::from([[1., 2.], [3., 4.]]);
    println!("\nOriginal matrix:\n{u}\nEchelon:\n{}", u.row_echelon());

    let u = Matrix::from([[1., 2.], [2., 4.]]);
    println!("\nOriginal matrix:\n{u}\nEchelon:\n{}", u.row_echelon());

    let u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    println!("\nOriginal matrix:\n{u}\nEchelon:\n{}", u.row_echelon());

    println!("\n=== Matrix Determinant Demonstration ===");
    let u = Matrix::from([[1., -1.], [-1., 1.]]);
    println!("Matrix:\n{u}\nDeterminant: {}", u.determinant());

    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!("\nMatrix:\n{u}\nDeterminant: {}", u.determinant());

    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    println!("\nMatrix:\n{u}\nDeterminant: {}", u.determinant());

    let u = Matrix::from([
        [8., 5., -2., 4.],
        [4., 2.5, 20., 4.],
        [8., 5., 1., 4.],
        [28., -4., 17., 1.],
    ]);

    println!("\nMatrix:\n{u}\nDeterminant: {}", u.determinant());
    println!("\n=== Matrix Inverse Demonstration ===");
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("Identity matrix:\n{u}\nInverse:\n{}", u.inverse().unwrap());

    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!(
        "\nDiagonal matrix:\n{u}\nInverse:\n{}",
        u.inverse().unwrap()
    );

    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    println!("\nGeneric matrix:\n{u}\nInverse:\n{}", u.inverse().unwrap());

    println!("\n=== Matrix Rank Demonstration ===");
    let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("Identity matrix:\n{u}\nRank: {}", u.rank());

    let mut u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
    println!("\nMatrix 3x4:\n{u}\nRank: {}", u.rank());

    let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
    println!("\nMatrix 4x3:\n{u}\nRank: {}", u.rank());
}
