use linear_algebra::{Matrix, Vector, angle_cos, cross_product, lerp, linear_combination};

fn main() {
    println!("=== Demonstração de Vector ===");
    let mut v1 = Vector::from([1, 2, 3]);
    let v2 = Vector::from([4, 5, 6]);

    println!("Vector 1: {v1}");
    println!("Vector 2: {v2}");

    v1.add_inline(&v2);
    println!("\nDepois da adição: {v1}");

    v1.sub(&v2);
    println!("Depois da subtração: {v1}");

    v1.scl(3);
    println!("Vetor 1 multiplicado por 3: {v1}");

    println!("Adição retornando um novo: {}", v1.add_new(&v2));

    println!("Subtração retornando um novo: {}", v1.sub_new(&v2));

    println!("Multiplicada por 3 retornando um novo: {}", v1.scl_new(3));

    println!("\n=== Demonstração de Matrix ===");
    let mut matrix1 = Matrix::from([[1, 2], [3, 4]]);
    let matrix2 = Matrix::from([[5, 6], [7, 8]]);

    println!("Matrix 1:\n{matrix1}");
    println!("\nMatrix 2:\n{matrix2}");

    matrix1.add(&matrix2);
    println!("\nDepois da adição:\n{matrix1}");

    matrix1.sub(&matrix2);
    println!("\nDepois da subtração:\n{matrix1}");

    matrix1.scl(3);
    println!("\nMatrix 1 multiplicada por 3:\n{matrix1}");

    println!(
        "\nAdição retornando um novo:\n{}",
        matrix1.add_new(&matrix2)
    );

    println!(
        "\nSubtração retornando um novo:\n{}",
        matrix1.sub_new(&matrix2)
    );

    println!(
        "\nMultiplicada por 3 retornando um novo:\n{}",
        matrix1.scl_new(3)
    );

    println!("\n=== Demonstração de Combinação linear ===");
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

    println!("\n=== Demonstração de Interpolação linear ===");
    println!(
        "Aplicando lerp em escalares: {}",
        lerp(1.2, 1.4, 0.5).unwrap()
    );

    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!(
        "\nAplicando lerp em vetores:\n{}",
        lerp(v1, v2, 0.4).unwrap()
    );

    let matrix1 = Matrix::from([[1., 2.], [3., 4.]]);
    let matrix2 = Matrix::from([[5., 6.], [7., 8.]]);
    println!(
        "\nAplicando lerp em matriz:\n{}",
        lerp(matrix1, matrix2, 0.3).unwrap()
    );

    println!("\n=== Demonstração de Produto Escalar (Dot Product) ===");

    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("Produto escalar entre {u} e {v}: {}", u.dot(&v));

    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("Produto escalar entre {u} e {v}: {}", u.dot(&v));

    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("Produto escalar entre {u} e {v}: {}", u.dot(&v));

    println!("\n=== Demonstração de Normas de Vetores ===");
    let u = Vector::from([0., 0., 0.]);
    println!(
        "Para o vetor {u}: norma-1 = {}, norma-2 = {}, norma-infinito = {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );

    let u = Vector::from([1., 2., 3.]);
    println!(
        "Para o vetor {u}: norma-1 = {}, norma-2 = {}, norma-infinito = {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );

    let u = Vector::from([-1., -2.]);
    println!(
        "Para o vetor {u}: norma-1 = {}, norma-2 = {}, norma-infinito = {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );

    println!("\n=== Demonstração do Cosseno do Ângulo entre Vetores ===");
    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("Cosseno do ângulo entre {u} e {v}: {}", angle_cos(&u, &v));

    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("Cosseno do ângulo entre {u} e {v}: {}", angle_cos(&u, &v));

    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);
    println!("Cosseno do ângulo entre {u} e {v}: {}", angle_cos(&u, &v));

    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("Cosseno do ângulo entre {u} e {v}: {}", angle_cos(&u, &v));

    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("Cosseno do ângulo entre {u} e {v}: {}", angle_cos(&u, &v));

    println!("\n=== Demonstração do Produto Vetorial (Cross Product) ===");
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!(
        "Produto vetorial entre {u} e {v}: {}",
        cross_product(&u, &v)
    );

    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!(
        "Produto vetorial entre {u} e {v}: {}",
        cross_product(&u, &v)
    );

    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!(
        "Produto vetorial entre {u} e {v}: {}",
        cross_product(&u, &v)
    );

    println!("\n=== Demonstração de Multiplicação Matrix x Vector ===");
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from([4., 2.]);
    println!(
        "Multiplicando a matriz identidade por {v}:\n{}",
        u.mul_vec(&v)
    );

    let u = Matrix::from([[2., 0.], [0., 2.]]);
    let v = Vector::from([4., 2.]);
    println!(
        "\nMultiplicando matriz de escala por {v}:\n{}",
        u.mul_vec(&v)
    );

    let u = Matrix::from([[2., -2.], [-2., 2.]]);
    let v = Vector::from([4., 2.]);
    println!(
        "\nMultiplicando matriz com elementos negativos por {v}:\n{}",
        u.mul_vec(&v)
    );

    println!("\n=== Demonstração de Multiplicação Matrix x Matrix ===");
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[1., 0.], [0., 1.]]);
    println!(
        "Multiplicando matriz identidade por matriz identidade:\n{}",
        u.mul_mat(&v)
    );

    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!(
        "\nMultiplicando matriz identidade por outra matriz:\n{}",
        u.mul_mat(&v)
    );

    let u = Matrix::from([[3., -5.], [6., 8.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("\nMultiplicando matrizes genéricas:\n{}", u.mul_mat(&v));

    println!("\n=== Demonstração do Traço de Matrizes ===");
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("Traço da matriz identidade:\n{u}\nTraço: {}", u.trace());

    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("\nTraço da matriz 3x3:\n{u}\nTraço: {}", u.trace());

    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    println!("\nTraço da matriz 3x3:\n{u}\nTraço: {}", u.trace());

    println!("\n=== Demonstração de Transposição de Matrizes ===");
    let u = Matrix::from([[1., 1.], [0., 0.]]);
    println!("Matriz original:\n{u}\nTransposta:\n{}", u.transpose());

    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.]]);
    println!("\nMatriz original:\n{u}\nTransposta:\n{}", u.transpose());

    let u = Matrix::from([[-2., -8.], [1., -23.], [0., 6.]]);
    println!("\nMatriz original:\n{u}\nTransposta:\n{}", u.transpose());

    println!("\n=== Demonstração de Resolução de sistemas com Matrizes ===");
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("\nMatriz original:\n{u}\nEchelon:\n{}", u.row_echelon());

    let u = Matrix::from([[1., 2.], [3., 4.]]);
    println!("\nMatriz original:\n{u}\nEchelon:\n{}", u.row_echelon());

    let u = Matrix::from([[1., 2.], [2., 4.]]);
    println!("\nMatriz original:\n{u}\nEchelon:\n{}", u.row_echelon());

    let u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    println!("\nMatriz original:\n{u}\nEchelon:\n{}", u.row_echelon());
}
