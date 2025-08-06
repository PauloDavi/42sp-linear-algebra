# Linear Algebra Library

Uma biblioteca Rust para operações de álgebra linear, incluindo vetores, matrizes e combinações lineares.

## Características

- **Vetores**: Operações de adição, subtração, multiplicação escalar e produto escalar
- **Matrizes**: Operações básicas de álgebra linear com matrizes
- **Combinações Lineares**: Cálculo de combinações lineares de vetores
- **Análise de Complexidade**: Ferramentas para análise Big O das operações
- **Benchmarks**: Medição de performance com diferentes tamanhos de entrada
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
    
    // Produto escalar
    let v3 = Vector::new(&[1, 0, 0]);
    let v4 = Vector::new(&[0, 1, 0]);
    let dot_product = v3.dot(&v4).unwrap();
    println!("v3 · v4: {}", dot_product); // 0
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
    m1.scl(2);
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
- `dot(&self, other: &Vector<K>)` - Calcula o produto escalar

#### Métodos Funcionais (Não Modificam o Original)

- `add_new(&self, other: &Vector<K>)` - Retorna um novo vetor com a soma
- `sub_new(&self, other: &Vector<K>)` - Retorna um novo vetor com a subtração
- `scalar_new(&self, a: K)` - Retorna um novo vetor multiplicado por escalar
- `norm_squared(&self)` - Calcula a norma ao quadrado

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
- `scl(&mut self, scalar: K)` - Multiplica por um escalar

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

## Análise de Complexidade Big O

Esta biblioteca inclui ferramentas para análise empírica da complexidade Big O das operações:

### Executando Análise Big O

```bash
# Execute o exemplo de análise Big O
cargo run --example big_o_analysis --features big_o_analysis
```

### Executando Benchmarks Detalhados

```bash
# Execute benchmarks com análise de scaling
cargo bench

# Gere relatórios HTML dos benchmarks
cargo bench -- --output-format html
```

### Complexidades Esperadas

| Operação | Complexidade | Descrição |
|----------|-------------|-----------|
| Adição de vetores | O(n) | Linear no tamanho do vetor |
| Subtração de vetores | O(n) | Linear no tamanho do vetor |
| Multiplicação escalar | O(n) | Linear no tamanho do vetor |
| Produto escalar | O(n) | Linear no tamanho do vetor |
| Combinação linear | O(k×n) | k = número de vetores, n = tamanho |

### Exemplo de Análise

```rust
use big_o::{big_o, Name};
use linear_algebra::Vector;

let complexities = big_o(
    Name::new("vector_addition"),
    || {
        move |n| {
            let v1 = Vector::new(&vec![1.0; n]);
            let v2 = Vector::new(&vec![2.0; n]);
            (v1, v2)
        }
    },
    |(mut v1, v2)| {
        v1.add(&v2).unwrap();
        v1
    },
);

for (name, complexity) in complexities {
    println!("{}: {}", name, complexity);
}
```

## Desenvolvimento

## Licença

Este projeto está licenciado sob as licenças MIT OU Apache-2.0.
