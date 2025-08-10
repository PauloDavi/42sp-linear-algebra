use linear_algebra_42::traits::{Magnitude, One, Zero};
use linear_algebra_42::{Complex, Matrix, Vector, angle_cos, linear_combination};

fn main() {
    println!("=== DEMONSTRATION WITH COMPLEX NUMBERS ===");
    println!("This example shows all operations using only complex numbers!\n");

    println!("=== Creating Complex Numbers ===");
    let z1 = Complex::new(3.0, 4.0);
    let z2 = Complex::new(1.0, -2.0);
    let z3 = Complex::new(-2.0, 1.0);
    let z4 = Complex::new(5.0, 0.0);
    let z5 = Complex::new(0.0, 3.0);

    println!("z1 = {z1}");
    println!("z2 = {z2}");
    println!("z3 = {z3}");
    println!("z4 = {z4}");
    println!("z5 = {z5}");

    println!("\n=== Arithmetic Operations with Complex Numbers ===");
    println!("z1 + z2 = {z1} + {z2} = {}", z1 + z2);

    println!("z1 * z2 = {z1} * {z2} = {}", z1 * z2);

    println!("z1 / z2 = {z1} / {z2} = {}", z1 / z2);

    println!("|z1| = |{}| = {}", z1, z1.magnitude());
    println!("conj(z1) = conj({}) = {}", z1, z1.conjugate());

    println!("\n=== Vectors of Complex Numbers ===");
    let mut v1 = Vector::from([z1, z2, z3]);
    let v2 = Vector::from([z4, z5, Complex::new(1.0, 1.0)]);

    println!("Vector v1 with complex components:");
    for (i, &z) in v1.iter().enumerate() {
        println!("  v1[{i}] = {z}");
    }

    println!("\nVector v2 with complex components:");
    for (i, &z) in v2.iter().enumerate() {
        println!("  v2[{i}] = {z}");
    }

    println!("\n=== Operations with Complex Vectors ===");

    let v_sum = v1.add_new(&v2);
    println!("Sum v1 + v2:");
    for (i, &z) in v_sum.iter().enumerate() {
        println!("  (v1 + v2)[{i}] = {z}");
    }

    v1.scl(Complex::new(2.0, 0.0));
    println!("\nVector v1 multiplied by 2:");
    for (i, &z) in v1.iter().enumerate() {
        println!("  (2 * v1)[{i}] = {z}");
    }

    println!("\n=== Complex Dot Product ===");
    let dot_product = v1.dot(&v2);
    println!("Dot product v1 · v2 = {dot_product}");
    println!("Magnitude of the dot product: {}", dot_product.magnitude());

    println!("\n=== Norms of Complex Vectors ===");
    let v_test = Vector::from([
        Complex::new(3.0, 4.0),
        Complex::new(0.0, 5.0),
        Complex::new(-1.0, 1.0),
    ]);

    println!("Test vector:");
    for (i, &z) in v_test.iter().enumerate() {
        println!("  v[{i}] = {z} (magnitude: {})", z.magnitude());
    }

    println!("Vector 1-norm: {}", v_test.norm_1());
    println!("Vector 2-norm: {}", v_test.norm());
    println!("Vector infinity norm: {}", v_test.norm_inf());

    println!("\n=== Angle Between Complex Vectors ===");
    let u_complex = Vector::from([Complex::new(1.0, 0.0), Complex::new(0.0, 1.0)]);
    let v_complex = Vector::from([Complex::new(0.0, 1.0), Complex::new(1.0, 0.0)]);

    println!("Vector u:");
    for (i, &z) in u_complex.iter().enumerate() {
        println!("  u[{i}] = {z}");
    }

    println!("Vector v:");
    for (i, &z) in v_complex.iter().enumerate() {
        println!("  v[{i}] = {z}");
    }

    let cos_angle = angle_cos(&u_complex, &v_complex);
    println!("Cosine of the angle between u and v: {cos_angle}");

    println!("\n=== Linear Combination with Complex Numbers ===");
    let e1 = Vector::from([Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]);
    let e2 = Vector::from([Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]);

    let coeff1 = Complex::new(2.0, 3.0);
    let coeff2 = Complex::new(-1.0, 2.0);

    println!("Complex basis:");
    println!("  e1 = [{}, {}]", e1[0], e1[1]);
    println!("  e2 = [{}, {}]", e2[0], e2[1]);

    println!("Coefficients:");
    println!("  c1 = {coeff1}");
    println!("  c2 = {coeff2}");

    let combination = linear_combination([e1, e2], [coeff1, coeff2]).unwrap();
    println!("Combination c1*e1 + c2*e2:");
    for (i, &z) in combination.iter().enumerate() {
        println!("  result[{i}] = {z}");
    }

    println!("\n=== Complex Number Matrices ===");
    let m1 = Matrix::from([
        [Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)],
        [Complex::new(0.0, 1.0), Complex::new(2.0, 0.0)],
    ]);

    let m2 = Matrix::from([
        [Complex::new(2.0, 0.0), Complex::new(-1.0, 1.0)],
        [Complex::new(1.0, -1.0), Complex::new(0.0, 2.0)],
    ]);

    println!("Matrix m1:");
    for i in 0..m1.rows() {
        print!("  [");
        for j in 0..m1.columns() {
            let z = m1[i][j];
            print!("{z}");
            if j < m1.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    println!("Matrix m2:");
    for i in 0..m2.rows() {
        print!("  [");
        for j in 0..m2.columns() {
            let z = m2[i][j];
            print!("{z}");
            if j < m2.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    println!("\n=== Operations with Complex Matrices ===");

    let m_sum = m1.add_new(&m2);
    println!("Sum m1 + m2:");
    for i in 0..m_sum.rows() {
        print!("  [");
        for j in 0..m_sum.columns() {
            let z = m_sum[i][j];
            print!("{z}");
            if j < m_sum.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    let m_product = m1.mul_mat(&m2);
    println!("Product m1 * m2:");
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

    println!("\n=== Matrix × Complex Vector ===");
    let complex_vector = Vector::from([Complex::new(1.0, 1.0), Complex::new(2.0, -1.0)]);

    println!("Vector:");
    for (i, &z) in complex_vector.iter().enumerate() {
        println!("  v[{i}] = {z}");
    }

    let result_vector = m1.mul_vec(&complex_vector);
    println!("Result m1 * v:");
    for (i, &z) in result_vector.iter().enumerate() {
        println!("  result[{i}] = {z}");
    }

    println!("\n=== Properties of Complex Matrices ===");

    let trace = m1.trace();
    println!("Trace of m1: {trace}");

    let m1_transpose = m1.transpose();
    println!("Transpose of m1:");
    for i in 0..m1_transpose.rows() {
        print!("  [");
        for j in 0..m1_transpose.columns() {
            let z = m1_transpose[i][j];
            print!("{z}");
            if j < m1_transpose.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    let determinant = m1.determinant();
    println!("Determinant of m1: {determinant}");
    println!("Magnitude of determinant: {}", determinant.magnitude());

    println!("\n=== Complex Identity Matrix ===");
    let identity = Matrix::from([
        [Complex::one(), Complex::zero()],
        [Complex::zero(), Complex::one()],
    ]);

    println!("Complex identity matrix:");
    for i in 0..identity.rows() {
        print!("  [");
        for j in 0..identity.columns() {
            let z = identity[i][j];
            print!("{z}");
            if j < identity.columns() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    let identity_product = m1.mul_mat(&identity);
    println!("Check: m1 * I = m1? {}", m1 == identity_product);
}
