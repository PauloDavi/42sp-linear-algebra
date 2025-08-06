# Linear Algebra Library

Uma biblioteca Rust para operações de álgebra linear, incluindo vetores, matrizes e combinações lineares.

## Características

- **Vetores**: Operações de adição, subtração e multiplicação escalar
- **Matrizes**: Operações básicas de álgebra linear com matrizes
- **Combinações Lineares**: Cálculo de combinações lineares de vetores
- **Segurança de Tipos**: Aproveitando o sistema de tipos do Rust para operações seguras
- **Genérico**: Funciona com qualquer tipo numérico que implemente os traits necessários

## Uso

Adicione esta dependência ao seu `Cargo.toml`:

```toml
[dependencies]
linear_algebra = "0.1.0"
```

### Exemplos

#### Operações com Vetores

```rust
use linear_algebra::Vector;

fn main() {
    // Criar vetores
    let mut v1 = Vector::new(&[1, 2, 3]);
    let v2 = Vector::new(&[4, 5, 6]);
    
    println!("v1: {}", v1); // [1, 2, 3]
    println!("v2: {}", v2); // [4, 5, 6]
    
    // Adição
    v1.add(&v2).unwrap();
    println!("v1 + v2: {}", v1); // [5, 7, 9]
    
    // Multiplicação escalar
    v1.scalar(2);
    println!("2 * v1: {}", v1); // [10, 14, 18]
}
```

#### Combinações Lineares

```rust
use linear_algebra::{Vector, linear_combination};

fn main() {
    // Vetores base
    let e1 = Vector::new(&[1.0, 0.0, 0.0]);
    let e2 = Vector::new(&[0.0, 1.0, 0.0]);
    let e3 = Vector::new(&[0.0, 0.0, 1.0]);
    
    // Coeficientes
    let coefficients = [2.0, -1.0, 0.5];
    
    // Calcular combinação linear: 2*e1 + (-1)*e2 + 0.5*e3
    let result = linear_combination(&[e1, e2, e3], &coefficients).unwrap();
    println!("Resultado: {}", result); // [2.0, -1.0, 0.5]
}
```

#### Operações com Matrizes

```rust
use linear_algebra::Matrix;

fn main() {
    // Criar matrizes
    let mut m1 = Matrix::new(&[&[1, 2], &[3, 4]]).unwrap();
    let m2 = Matrix::new(&[&[5, 6], &[7, 8]]).unwrap();
    
    println!("Matriz 1:\n{}", m1);
    println!("Matriz 2:\n{}", m2);
    
    // Adição de matrizes
    m1.add(&m2).unwrap();
    println!("m1 + m2:\n{}", m1);
    
    // Multiplicação escalar
    m1.scalar(2);
    println!("2 * m1:\n{}", m1);
}
```

## API

### Vector<K>

#### Métodos Principais

- `new(data: &[K])` - Cria um novo vetor
- `len()` - Retorna o comprimento do vetor
- `add(&mut self, other: &Vector<K>)` - Adiciona outro vetor (modifica o vetor atual)
- `sub(&mut self, other: &Vector<K>)` - Subtrai outro vetor (modifica o vetor atual)
- `scalar(&mut self, a: K)` - Multiplica por um escalar (modifica o vetor atual)

#### Métodos Funcionais (Não Modificam o Original)

- `add_new(&self, other: &Vector<K>)` - Retorna um novo vetor com a soma
- `sub_new(&self, other: &Vector<K>)` - Retorna um novo vetor com a subtração
- `scalar_new(&self, a: K)` - Retorna um novo vetor multiplicado por escalar

#### Acesso a Elementos

- `get(index: usize)` - Acesso seguro a elemento
- `[index]` - Indexação direta (pode causar panic)
- `iter()` - Iterador sobre os elementos

### Matrix<K>

#### Métodos Principais

- `new(data: &[&[K]])` - Cria uma nova matriz
- `shape()` - Retorna as dimensões (linhas, colunas)
- `add(&mut self, other: &Matrix<K>)` - Adiciona outra matriz
- `sub(&mut self, other: &Matrix<K>)` - Subtrai outra matriz
- `scalar(&mut self, scalar: K)` - Multiplica por um escalar

### Funções Utilitárias

- `linear_combination(vectors: &[Vector<K>], coefficients: &[K])` - Calcula combinação linear

## Requisitos de Trait

Os tipos genéricos `K` devem implementar:

- `Copy` - Para permitir cópia eficiente
- `Add<Output = K> + AddAssign` - Para adição
- `Sub<Output = K> + SubAssign` - Para subtração
- `Mul<Output = K> + MulAssign` - Para multiplicação
- `Default` - Para criar elementos zero
- `Display` - Para impressão (opcional)

## Tratamento de Erros

A biblioteca usa tipos `Result` para operações que podem falhar:

- `VectorError::DimensionMismatch` - Quando vetores têm dimensões incompatíveis
- `VectorError::LinearCombinationDimensionMismatch` - Quando o número de vetores e coeficientes não coincidem
- `MatrixError::DimensionMismatch` - Quando matrizes têm dimensões incompatíveis
- `MatrixError::RowsLengthMismatch` - Quando as linhas da matriz têm comprimentos diferentes

## Desenvolvimento

Para executar os testes:

```bash
cargo test
```

Para executar o exemplo:

```bash
cargo run
```

Para verificar a documentação:

```bash
cargo doc --open
```

## Licença

Este projeto está licenciado sob as licenças MIT.
