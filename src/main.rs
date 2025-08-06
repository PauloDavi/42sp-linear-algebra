use linear_algebra::{linear_combination, Matrix, Vector};

fn main() {
    println!("=== Demonstração de Vector ===");
    let mut v1 = Vector::new(&[1, 2, 3]);
    let v2 = Vector::new(&[4, 5, 6]);

    println!("Vector 1: {v1}");
    println!("Vector 2: {v2}");

    v1.add(&v2).unwrap();
    println!("\nDepois da adição: {v1}");

    v1.sub(&v2).unwrap();
    println!("Depois da subtração: {v1}");

    v1.scalar(3);
    println!("Vetor 1 multiplicado por 3: {v1}");

    println!("Adição retornando um novo: {}", v1.add_new(&v2).unwrap());

    println!("Subtração retornando um novo: {}", v1.sub_new(&v2).unwrap());

    println!(
        "Multiplicada por 3 retornando um novo: {}",
        v1.scalar_new(3)
    );

    println!("\n=== Demonstração de Matrix ===");
    let mut matrix1 = Matrix::new(&[&[1, 2], &[3, 4]]).unwrap();
    let matrix2 = Matrix::new(&[&[5, 6], &[7, 8]]).unwrap();

    println!("Matrix 1:\n{matrix1}");
    println!("\nMatrix 2:\n{matrix2}");

    matrix1.add(&matrix2).unwrap();
    println!("\nDepois da adição:\n{matrix1}");

    matrix1.sub(&matrix2).unwrap();
    println!("\nDepois da subtração:\n{matrix1}");

    matrix1.scalar(3);
    println!("\nMatrix 1 multiplicada por 3:\n{matrix1}");

    println!(
        "\nAdição retornando um novo:\n{}",
        matrix1.add_new(&matrix2).unwrap()
    );

    println!(
        "\nSubtração retornando um novo:\n{}",
        matrix1.sub_new(&matrix2).unwrap()
    );

    println!(
        "\nMultiplicada por 3 retornando um novo:\n{}",
        matrix1.scalar_new(3)
    );

    println!("\n=== Demonstração de Combinação linear ===");
    let e1 = Vector::new(&[1., 0., 0.]);
    let e2 = Vector::new(&[0., 1., 0.]);
    let e3 = Vector::new(&[0., 0., 1.]);
    println!("e1: {e1}");
    println!("e2: {e2}");
    println!("e3: {e3}");

    let coefficients = [10., -2., 0.5];
    let result = linear_combination(&[e1, e2, e3], &coefficients).unwrap();
    println!(
        "({} * e1) + ({} * e2) + ({} * e3) = {result}",
        coefficients[0], coefficients[1], coefficients[2]
    );

    let v1 = Vector::new(&[1., 2., 3.]);
    let v2 = Vector::new(&[0., 10., -100.]);
    println!("\nv1: {v1}");
    println!("v2: {v2}");

    let coefficients = [10., -2.];
    let result = linear_combination(&[v1, v2], &coefficients).unwrap();
    println!(
        "({} * v1) + ({} * v2) = {result}",
        coefficients[0], coefficients[1]
    );
}
