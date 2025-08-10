use linear_algebra::traits::{Magnitude, One, Zero};
use linear_algebra::{Complex, Matrix, Vector, angle_cos, linear_combination};

fn main() {
    println!("=== DEMONSTRAÇÃO COM NÚMEROS COMPLEXOS ===");
    println!("Este exemplo mostra todas as operações usando apenas números complexos!\n");

    println!("=== Criação de Números Complexos ===");
    let z1 = Complex::new(3.0, 4.0);
    let z2 = Complex::new(1.0, -2.0);
    let z3 = Complex::new(-2.0, 1.0);
    let z4 = Complex::new(5.0, 0.0);
    let z5 = Complex::new(0.0, 3.0);

    println!("z1 = {}", z1);
    println!("z2 = {}", z2);
    println!("z3 = {}", z3);
    println!("z4 = {}", z4);
    println!("z5 = {}", z5);

    println!("\n=== Operações Aritméticas com Complexos ===");
    println!("z1 + z2 = {} + {} = {}", z1, z2, z1 + z2);

    println!("z1 * z2 = {} * {} = {}", z1, z2, z1 * z2);

    println!("z1 / z2 = {} / {} = {}", z1, z2, z1 / z2);

    println!("|z1| = |{}| = {}", z1, z1.magnitude());
    println!("conj(z1) = conj({}) = {}", z1, z1.conjugate());

    println!("\n=== Vetores de Números Complexos ===");
    let mut v1 = Vector::from([z1, z2, z3]);
    let v2 = Vector::from([z4, z5, Complex::new(1.0, 1.0)]);

    println!("Vetor v1 com componentes complexas:");
    for (i, &z) in v1.iter().enumerate() {
        println!("  v1[{}] = {}", i, z);
    }

    println!("\nVetor v2 com componentes complexas:");
    for (i, &z) in v2.iter().enumerate() {
        println!("  v2[{}] = {}", i, z);
    }

    println!("\n=== Operações com Vetores Complexos ===");

    let v_sum = v1.add_new(&v2);
    println!("Soma v1 + v2:");
    for (i, &z) in v_sum.iter().enumerate() {
        println!("  (v1 + v2)[{}] = {}", i, z);
    }

    v1.scl(Complex::new(2.0, 0.0));
    println!("\ntor v1 multiplicado por 2:");
    for (i, &z) in v1.iter().enumerate() {
        println!("  (2 * v1)[{}] = {}", i, z);
    }

    println!("\n=== Produto Escalar Complexo ===");
    let dot_product = v1.dot(&v2);
    println!("Produto escalar v1 · v2 = {}", dot_product);
    println!("Magnitude do produto escalar: {}", dot_product.magnitude());

    println!("\n=== Normas de Vetores Complexos ===");
    let v_test = Vector::from([
        Complex::new(3.0, 4.0),
        Complex::new(0.0, 5.0),
        Complex::new(-1.0, 1.0),
    ]);

    println!("Vetor de teste:");
    for (i, &z) in v_test.iter().enumerate() {
        println!("  v[{}] = {} (magnitude: {})", i, z, z.magnitude());
    }

    println!("Norma-1 do vetor: {}", v_test.norm_1());
    println!("Norma-2 do vetor: {}", v_test.norm());
    println!("Norma-infinito do vetor: {}", v_test.norm_inf());

    println!("\n=== Ângulo entre Vetores Complexos ===");
    let u_complex = Vector::from([Complex::new(1.0, 0.0), Complex::new(0.0, 1.0)]);
    let v_complex = Vector::from([Complex::new(0.0, 1.0), Complex::new(1.0, 0.0)]);

    println!("Vetor u:");
    for (i, &z) in u_complex.iter().enumerate() {
        println!("  u[{}] = {}", i, z);
    }

    println!("Vetor v:");
    for (i, &z) in v_complex.iter().enumerate() {
        println!("  v[{}] = {}", i, z);
    }

    let cos_angle = angle_cos(&u_complex, &v_complex);
    println!("Cosseno do ângulo entre u e v: {}", cos_angle);

    println!("\n=== Combinação Linear com Complexos ===");
    let e1 = Vector::from([Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]);
    let e2 = Vector::from([Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]);

    let coeff1 = Complex::new(2.0, 3.0);
    let coeff2 = Complex::new(-1.0, 2.0);

    println!("Base complexa:");
    println!("  e1 = [{}, {}]", e1[0], e1[1]);
    println!("  e2 = [{}, {}]", e2[0], e2[1]);

    println!("Coeficientes:");
    println!("  c1 = {}", coeff1);
    println!("  c2 = {}", coeff2);

    let combination = linear_combination([e1, e2], [coeff1, coeff2]).unwrap();
    println!("Combinação c1*e1 + c2*e2:");
    for (i, &z) in combination.iter().enumerate() {
        println!("  resultado[{}] = {}", i, z);
    }

    println!("\n=== Matrizes de Números Complexos ===");
    let m1 = Matrix::from([
        [Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)],
        [Complex::new(0.0, 1.0), Complex::new(2.0, 0.0)],
    ]);

    let m2 = Matrix::from([
        [Complex::new(2.0, 0.0), Complex::new(-1.0, 1.0)],
        [Complex::new(1.0, -1.0), Complex::new(0.0, 2.0)],
    ]);

    println!("Matriz m1:");
    for i in 0..m1.rows() {
        print!("  [");
        for j in 0..m1.columns() {
            let z = m1[i][j];
            print!("{}", z);
            if j < m1.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    println!("Matriz m2:");
    for i in 0..m2.rows() {
        print!("  [");
        for j in 0..m2.columns() {
            let z = m2[i][j];
            print!("{}", z);
            if j < m2.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    println!("\n=== Operações com Matrizes Complexas ===");

    let m_sum = m1.add_new(&m2);
    println!("Soma m1 + m2:");
    for i in 0..m_sum.rows() {
        print!("  [");
        for j in 0..m_sum.columns() {
            let z = m_sum[i][j];
            print!("{}", z);
            if j < m_sum.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    let m_product = m1.mul_mat(&m2);
    println!("Produto m1 * m2:");
    for i in 0..m_product.rows() {
        print!("  [");
        for j in 0..m_product.columns() {
            let z = m_product[i][j];
            print!("{z}");
            if j < m_product.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    println!("\n=== Matriz X Vetor Complexo ===");
    let complex_vector = Vector::from([Complex::new(1.0, 1.0), Complex::new(2.0, -1.0)]);

    println!("Vetor:");
    for (i, &z) in complex_vector.iter().enumerate() {
        println!("  v[{}] = {}", i, z);
    }

    let result_vector = m1.mul_vec(&complex_vector);
    println!("Resultado m1 * v:");
    for (i, &z) in result_vector.iter().enumerate() {
        println!("  resultado[{}] = {}", i, z);
    }

    println!("\n=== Propriedades de Matrizes Complexas ===");

    let trace = m1.trace();
    println!("Traço de m1: {}", trace);

    let m1_transpose = m1.transpose();
    println!("Transposta de m1:");
    for i in 0..m1_transpose.rows() {
        print!("  [");
        for j in 0..m1_transpose.columns() {
            let z = m1_transpose[i][j];
            print!("{}", z);
            if j < m1_transpose.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    let determinant = m1.determinant();
    println!("Determinante de m1: {}", determinant);
    println!("Magnitude do determinante: {}", determinant.magnitude());

    println!("\n=== Matriz Identidade Complexa ===");
    let identity = Matrix::from([
        [Complex::one(), Complex::zero()],
        [Complex::zero(), Complex::one()],
    ]);

    println!("Matriz identidade complexa:");
    for i in 0..identity.rows() {
        print!("  [");
        for j in 0..identity.columns() {
            let z = identity[i][j];
            print!("{}", z);
            if j < identity.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    let identity_product = m1.mul_mat(&identity);
    println!("Verificação: m1 * I = m1? {}", m1 == identity_product);
}
