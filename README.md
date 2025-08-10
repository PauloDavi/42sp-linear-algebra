# Linear Algebra Library

A complete Rust library for linear algebra operations, based on the 42 School Matrix project. Features vectors, matrices, complex numbers, and comprehensive trait system with type safety and performance.

## Key Features

### 🧮 **Vectors & Matrices**
- Basic operations: add, subtract, scalar multiplication
- Advanced operations: dot product, cross product, matrix multiplication
- Mathematical functions: norms, determinant, inverse, rank, transpose
- Complex number support with conjugation operations

### 🔢 **Complex Numbers & New Traits**
- **Complex**: Full complex number arithmetic with real/imaginary parts
- **Conjugate trait**: Complex conjugation for numbers, vectors, and matrices
- **Magnitude trait**: Unified magnitude calculation across all types
- **Zero/One/Negative traits**: Mathematical identity and operations

### 🚀 **Performance & Safety**
- Generic implementation supporting any numeric type
- Type-safe operations with comprehensive error handling
- Optimized algorithms with in-place and functional variants
- Zero-dependency library using only Rust std

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
linear_algebra_42 = "0.1.0"
```

### Basic Examples

```rust
use linear_algebra_42::{Vector, Matrix, Complex, linear_combination, lerp};

// Vectors
let mut v1 = Vector::from([1, 2, 3]);
let v2 = Vector::from([4, 5, 6]);
v1.add_inline(&v2);  // [5, 7, 9]
println!("Dot product: {}", v1.dot(&v2));

// Matrices  
let m1 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
let m2 = Matrix::from([[5.0, 6.0], [7.0, 8.0]]);
let result = m1.mul_mat(&m2);
println!("Determinant: {}", m1.determinant());

// Complex Numbers
let z1 = Complex::new(3.0, 4.0);
let z2 = Complex::new(1.0, -2.0);
let product = z1 * z2;
println!("Conjugate: {}", z1.conjugate());

// Advanced Operations
let vectors = [Vector::from([1.0, 0.0]), Vector::from([0.0, 1.0])];
let result = linear_combination(vectors, [2.0, 3.0])?;
let interpolated = lerp(v1, v2, 0.5)?;
```

## Core API

### Vector<T>
- **Creation**: `Vector::from([1, 2, 3])`, `Vector::zeros(n)`
- **Operations**: `add_inline()`, `sub()`, `scl()`, `dot()`, `cross_product()`
- **Norms**: `norm_1()`, `norm()`, `norm_inf()`
- **Functional**: `add_new()`, `sub_new()`, `scl_new()`

### Matrix<T>  
- **Creation**: `Matrix::from([[1, 2], [3, 4]])`, `Matrix::zeros(rows, cols)`
- **Operations**: `add()`, `sub()`, `scl()`, `mul_vec()`, `mul_mat()`
- **Linear Algebra**: `transpose()`, `determinant()`, `inverse()`, `rank()`, `trace()`

### Complex
- **Creation**: `Complex::new(real, imag)`, `Complex::real(x)`, `Complex::imag(x)`  
- **Operations**: Standard arithmetic (`+`, `-`, `*`, `/`), `conjugate()`, `magnitude()`
- **Traits**: Implements `Conjugate`, `Magnitude`, `Zero`, `One`

### Utility Functions
- `linear_combination(vectors, coefficients)` - Linear combinations
- `lerp(start, end, t)` - Linear interpolation  
- `angle_cos(u, v)` - Cosine of angle between vectors
- `cross_product(u, v)` - 3D cross product

## Trait System

### Mathematical Traits
- **`Conjugate`**: Complex conjugation for `Complex`, `Vector<Complex>`, `Matrix<Complex>`
- **`Magnitude`**: Unified magnitude calculation (`abs()` for numbers, `norm()` for vectors)  
- **`Zero`**: Additive identity (`zero()` method)
- **`One`**: Multiplicative identity (`one()` method)
- **`Negative`**: Additive inverse (`negative()` method)

### Type Requirements
Most operations require combinations of: `Copy`, `Clone`, `Add`, `Sub`, `Mul`, `Div`, `Default`, `PartialEq`, `PartialOrd`

## Testing & Documentation

```bash
cargo test                     # Run all tests
cargo doc --open               # Generate and open documentation  
cargo run --example basic      # Run basic example
cargo run --example complex    # Run example with complex numbers
```

## Project Context

Implementation of the **42 School Matrix project** covering fundamental linear algebra concepts for computer graphics, machine learning, and engineering applications.

**Features implemented**: Vector operations, matrix algebra, linear systems, complex numbers with conjugation, comprehensive trait system, and optimized algorithms.

## License

MIT License - see `LICENSE-MIT` for details.
