# Linear Algebra Library

Uma biblioteca Rust completa para operações de álgebra linear, baseada no projeto Matrix da 42 School. Implementa vetores, matrizes e todas as operações fundamentais de álgebra linear com segurança de tipos e performance.

## Características Principais

### 🧮 **Vetores (Vector<K>)**
- **Operações Básicas**: Adição, subtração e multiplicação escalar
- **Produto Escalar**: Cálculo do dot product entre vetores
- **Normas**: Norma L1, L2 (Euclidiana) e norma infinito
- **Produto Vetorial**: Cross product para vetores 3D
- **Métodos Funcionais**: Versões que retornam novos vetores sem modificar os originais

### 📊 **Matrizes (Matrix<K>)**
- **Operações Básicas**: Adição, subtração e multiplicação escalar
- **Multiplicação**: Matrix × Vector e Matrix × Matrix
- **Transposição**: Conversão de linhas em colunas
- **Determinante**: Cálculo para matrizes quadradas (1×1 até 4×4)
- **Inversa**: Cálculo da matriz inversa usando eliminação Gaussiana
- **Traço**: Soma dos elementos da diagonal principal
- **Rank**: Posto da matriz usando eliminação Gaussiana
- **Forma Escalonada**: Redução à forma escalonada reduzida

### 🔧 **Operações Avançadas**
- **Combinações Lineares**: Cálculo de combinações lineares de vetores
- **Interpolação Linear (LERP)**: Interpolação entre escalares, vetores e matrizes
- **Cálculo de Ângulos**: Cosseno do ângulo entre vetores
- **Resolução de Sistemas**: Através da forma escalonada reduzida

### 🛡️ **Características Técnicas**
- **Segurança de Tipos**: Sistema de tipos do Rust para operações seguras
- **Genérico**: Funciona com qualquer tipo numérico que implemente os traits necessários
- **Tratamento de Erros**: Tipos `Result` para operações que podem falhar
- **Performance**: Implementações otimizadas com iteradores e operações in-place
- **Display Personalizado**: Formatação legível para vetores e matrizes

## Uso

Adicione esta dependência ao seu `Cargo.toml`:

```toml
[dependencies]
linear_algebra = "0.1.0"
```

### Exemplos Básicos

#### Operações com Vetores

```rust
use linear_algebra::Vector;

fn main() {
    // Criar vetores
    let mut v1 = Vector::from([1, 2, 3]);
    let v2 = Vector::from([4, 5, 6]);
    
    println!("v1: {}", v1); // [1, 2, 3]
    println!("v2: {}", v2); // [4, 5, 6]
    
    // Adição in-place
    v1.add_inline(&v2);
    println!("v1 + v2: {}", v1); // [5, 7, 9]
    
    // Multiplicação escalar
    v1.scl(2);
    println!("2 * v1: {}", v1); // [10, 14, 18]
    
    // Versões funcionais (retornam novos vetores)
    let v3 = v1.add_new(&v2);
    let v4 = v1.scl_new(3);
}
```

#### Produto Escalar e Normas

```rust
use linear_algebra::Vector;

fn main() {
    let u = Vector::from([1.0, 2.0, 3.0]);
    let v = Vector::from([4.0, 5.0, 6.0]);
    
    // Produto escalar
    println!("u · v = {}", u.dot(&v)); // 32.0
    
    // Normas
    println!("Norma L1: {}", u.norm_1());    // 6.0
    println!("Norma L2: {}", u.norm());      // 3.741...
    println!("Norma ∞: {}", u.norm_inf());   // 3.0
}
```

#### Produto Vetorial e Ângulos

```rust
use linear_algebra::{Vector, cross_product, angle_cos};

fn main() {
    let u = Vector::from([1.0, 0.0, 0.0]);
    let v = Vector::from([0.0, 1.0, 0.0]);
    
    // Produto vetorial (apenas para vetores 3D)
    let cross = cross_product(&u, &v);
    println!("u × v = {}", cross); // [0, 0, 1]
    
    // Cosseno do ângulo entre vetores
    let cos_angle = angle_cos(&u, &v);
    println!("cos(θ) = {}", cos_angle); // 0.0 (90 graus)
}
```

#### Combinações Lineares

```rust
use linear_algebra::{Vector, linear_combination};

fn main() {
    // Vetores base
    let e1 = Vector::from([1.0, 0.0, 0.0]);
    let e2 = Vector::from([0.0, 1.0, 0.0]);
    let e3 = Vector::from([0.0, 0.0, 1.0]);
    
    // Coeficientes
    let coefficients = [2.0, -1.0, 0.5];
    
    // Calcular: 2*e1 + (-1)*e2 + 0.5*e3
    let result = linear_combination([e1, e2, e3], coefficients).unwrap();
    println!("Resultado: {}", result); // [2.0, -1.0, 0.5]
}
```

#### Interpolação Linear (LERP)

```rust
use linear_algebra::{Vector, Matrix, lerp};

fn main() {
    // Interpolação de escalares
    let result = lerp(1.0, 2.0, 0.5).unwrap(); // 1.5
    
    // Interpolação de vetores
    let v1 = Vector::from([0.0, 0.0]);
    let v2 = Vector::from([10.0, 20.0]);
    let interpolated = lerp(v1, v2, 0.3).unwrap(); // [3.0, 6.0]
    
    // Interpolação de matrizes
    let m1 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
    let m2 = Matrix::from([[5.0, 6.0], [7.0, 8.0]]);
    let interpolated_matrix = lerp(m1, m2, 0.25).unwrap();
}
```

### Exemplos Avançados com Matrizes

#### Operações Básicas com Matrizes

```rust
use linear_algebra::Matrix;

fn main() {
    // Criar matrizes
    let mut m1 = Matrix::from([[1, 2], [3, 4]]);
    let m2 = Matrix::from([[5, 6], [7, 8]]);
    
    println!("Matriz 1:\n{}", m1);
    println!("Matriz 2:\n{}", m2);
    
    // Adição e multiplicação escalar
    m1.add(&m2);
    m1.scl(2);
    
    // Transposição
    let transposed = m1.transpose();
    println!("Transposta:\n{}", transposed);
}
```

#### Multiplicação Matrix × Vector e Matrix × Matrix

```rust
use linear_algebra::{Matrix, Vector};

fn main() {
    let matrix = Matrix::from([[2.0, 1.0], [1.0, 3.0]]);
    let vector = Vector::from([4.0, 2.0]);
    
    // Multiplicação Matrix × Vector
    let result_vec = matrix.mul_vec(&vector);
    println!("A * v = {}", result_vec);
    
    // Multiplicação Matrix × Matrix
    let matrix2 = Matrix::from([[1.0, 2.0], [3.0, 1.0]]);
    let result_mat = matrix.mul_mat(&matrix2);
    println!("A * B =\n{}", result_mat);
}
```

#### Determinante e Matriz Inversa

```rust
use linear_algebra::Matrix;

fn main() {
    let matrix = Matrix::from([[2.0, 1.0, 0.0], 
                               [1.0, 3.0, 1.0], 
                               [0.0, 1.0, 2.0]]);
    
    // Cálculo do determinante
    let det = matrix.determinant();
    println!("Determinante: {}", det);
    
    // Cálculo da matriz inversa
    match matrix.inverse() {
        Ok(inverse) => println!("Matriz inversa:\n{}", inverse),
        Err(_) => println!("Matriz não é invertível"),
    }
    
    // Traço da matriz
    let trace = matrix.trace();
    println!("Traço: {}", trace);
}
```

#### Resolução de Sistemas Lineares

```rust
use linear_algebra::Matrix;

fn main() {
    // Sistema: 2x + y = 5, x + 3y = 8
    // Matriz aumentada: [2 1 | 5]
    //                   [1 3 | 8]
    let mut system = Matrix::from([[2.0, 1.0, 5.0], 
                                   [1.0, 3.0, 8.0]]);
    
    println!("Sistema original:\n{}", system);
    
    // Forma escalonada reduzida
    let solved = system.row_echelon();
    println!("Forma escalonada:\n{}", solved);
    
    // Calcular o rank
    let rank = system.rank();
    println!("Rank da matriz: {}", rank);
}
```

## API Completa

### Vector<K>

#### Construtores e Conversões
- `Vector::from(data: &[K])` - Cria um vetor a partir de um slice
- `Vector::from(array: [K; N])` - Cria um vetor a partir de um array
- `Vector::zeros(len: usize)` - Cria um vetor preenchido com valores padrão

#### Métodos de Informação
- `len()` - Retorna o comprimento do vetor
- `iter()` - Iterador sobre os elementos (imutável)
- `iter_mut()` - Iterador sobre os elementos (mutável)
- `data()` - Acesso direto ao Vec<K> interno

#### Operações Aritméticas (In-place)
- `add_inline(&mut self, other: &Vector<K>)` - Adiciona outro vetor
- `sub(&mut self, other: &Vector<K>)` - Subtrai outro vetor
- `scl(&mut self, scalar: K)` - Multiplica por um escalar

#### Operações Aritméticas (Funcionais)
- `add_new(&self, other: &Vector<K>)` - Retorna novo vetor com a soma
- `sub_new(&self, other: &Vector<K>)` - Retorna novo vetor com a subtração
- `scl_new(&self, scalar: K)` - Retorna novo vetor multiplicado por escalar

#### Produtos e Normas
- `dot(&self, other: &Vector<K>)` - Produto escalar
- `norm_1(&self)` - Norma L1 (Manhattan)
- `norm(&self)` - Norma L2 (Euclidiana)
- `norm_inf(&self)` - Norma infinito (máximo absoluto)

#### Acesso a Elementos
- `[index]` - Indexação direta
- `to_vec()` - Converte para Vec<K>

#### Traits Implementados
- `Add`, `Sub`, `Mul<K>` - Operadores aritméticos
- `Index`, `IndexMut` - Indexação
- `Display` - Formatação para impressão
- `Clone`, `PartialEq`, `Debug` - Traits padrão

### Matrix<K>

#### Construtores e Conversões
- `Matrix::from(array: [[K; C]; R])` - Cria matriz a partir de array 2D
- `Matrix::zeros(rows, columns)` - Cria matriz preenchida com valores padrão

#### Métodos de Informação
- `shape()` - Retorna (linhas, colunas)
- `is_square()` - Verifica se é matriz quadrada
- `iter()` - Iterador sobre as linhas
- `iter_mut()` - Iterador mutável sobre as linhas

#### Operações Aritméticas (In-place)
- `add(&mut self, other: &Matrix<K>)` - Adiciona outra matriz
- `sub(&mut self, other: &Matrix<K>)` - Subtrai outra matriz
- `scl(&mut self, scalar: K)` - Multiplica por um escalar

#### Operações Aritméticas (Funcionais)
- `add_new(&self, other: &Matrix<K>)` - Retorna nova matriz com a soma
- `sub_new(&self, other: &Matrix<K>)` - Retorna nova matriz com a subtração
- `scl_new(&self, scalar: K)` - Retorna nova matriz multiplicada por escalar

#### Operações de Álgebra Linear
- `transpose()` - Retorna a matriz transposta
- `mul_vec(&self, vector: &Vector<K>)` - Multiplica matriz por vetor
- `mul_mat(&self, matrix: &Matrix<K>)` - Multiplica matriz por matriz
- `trace()` - Retorna o traço (soma da diagonal principal)
- `determinant()` - Calcula o determinante (matrizes 1×1 a 4×4)
- `inverse()` - Calcula a matriz inversa
- `rank(&mut self)` - Calcula o posto da matriz
- `row_echelon()` - Retorna a forma escalonada reduzida

#### Operações Auxiliares
- `sub_matrix(remove_row, remove_col)` - Remove linha e coluna específicas

#### Acesso a Elementos
- `[row_index]` - Acesso a uma linha completa
- `[row_index][col_index]` - Acesso a elemento específico

#### Traits Implementados
- `Add`, `Sub`, `Mul<K>` - Operadores aritméticos
- `Index`, `IndexMut` - Indexação
- `Display` - Formatação para impressão
- `Clone`, `PartialEq`, `Debug` - Traits padrão

### Funções Utilitárias

#### `linear_combination<K, const N: usize>(vectors: [Vector<K>; N], coefficients: [K; N])`
Calcula a combinação linear de vetores com os coeficientes dados.

**Exemplo:**
```rust
let result = linear_combination([v1, v2, v3], [2.0, -1.0, 0.5])?;
// Retorna: 2.0*v1 + (-1.0)*v2 + 0.5*v3
```

#### `lerp<K>(u: K, v: K, t: f32)`
Interpolação linear entre dois valores (escalares, vetores ou matrizes).

**Fórmula:** `u * (1 - t) + v * t`, onde `t ∈ [0, 1]`

#### `angle_cos<K>(u: &Vector<K>, v: &Vector<K>)`
Calcula o cosseno do ângulo entre dois vetores.

**Fórmula:** `(u · v) / (||u|| * ||v||)`

#### `cross_product<K>(u: &Vector<K>, v: &Vector<K>)`
Calcula o produto vetorial entre dois vetores 3D.

**Resultado:** Vetor perpendicular ao plano formado por u e v

## Requisitos de Trait

Os tipos genéricos `K` devem implementar diferentes combinações de traits dependendo da operação:

### Traits Básicos (Sempre Necessários)
- `Copy` - Para permitir cópia eficiente dos valores
- `Clone` - Para operações que precisam clonar dados

### Traits Aritméticos
- `Add<Output = K>` - Para operações de adição
- `Sub<Output = K>` - Para operações de subtração  
- `Mul<Output = K>` - Para operações de multiplicação
- `Div<Output = K>` - Para operações de divisão (inversas, normalização)
- `Neg<Output = K>` - Para negação (eliminação Gaussiana)

### Traits de Comparação e Conversão
- `Default` - Para criar elementos zero/neutros
- `PartialEq` - Para comparações de igualdade
- `PartialOrd` - Para comparações de ordem (rank, pivoteamento)
- `From<T>` e `Into<T>` - Para conversões entre tipos numéricos

### Traits Específicos por Funcionalidade

#### Para Produto Escalar e Normas
```rust
K: Copy + Default + Add<Output = K> + Mul<Output = K> + Into<f32>
```

#### Para Determinante
```rust
K: Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + From<i8>
```

#### Para Matriz Inversa
```rust
K: Copy + Default + Add<Output = K> + Sub<Output = K> + 
   Div<Output = K> + Mul<Output = K> + From<i8> + PartialEq
```

#### Para Rank
```rust
K: Copy + Default + PartialOrd + Sub<Output = K> + 
   Div<Output = K> + Mul<Output = K> + From<f64> + Into<f64>
```

## Tratamento de Erros

A biblioteca usa tipos `Result` para operações que podem falhar:

### `LinearCombinationError`
- `CoefficientsDimensionMismatch` - Número de vetores e coeficientes não coincidem
- `VectorsDimensionMismatch` - Vetores têm dimensões incompatíveis

### `InterpolationError`
- `InvalidParameterT` - Parâmetro t fora do intervalo [0, 1]

### Operações que Retornam `Result<T, ()>`
- `Matrix::inverse()` - Falha se a matriz não for invertível
- Algumas operações de matriz podem retornar erro para matrizes não quadradas ou singulares

## Contexto: Projeto Matrix da 42 School

Esta biblioteca é uma implementação completa do projeto **Matrix: An Introduction to Linear Algebra** da 42 School. O projeto abrange os conceitos fundamentais de álgebra linear necessários para computação gráfica, machine learning e engenharia:

### Exercícios Implementados

1. **Ex00-01**: Operações básicas (soma, subtração, multiplicação escalar)
2. **Ex02**: Interpolação linear (LERP)
3. **Ex03**: Produto escalar (dot product)
4. **Ex04**: Normas de vetores (L1, L2, infinito)
5. **Ex05**: Cosseno do ângulo entre vetores
6. **Ex06**: Produto vetorial (cross product)
7. **Ex07**: Multiplicação matriz-vetor
8. **Ex08**: Multiplicação matriz-matriz
9. **Ex09**: Transposição de matriz
10. **Ex10**: Forma escalonada reduzida
11. **Ex11**: Determinante
12. **Ex12**: Matriz inversa
13. **Ex13**: Rank (posto) da matriz

### Aplicações Práticas

- **Computação Gráfica**: Transformações 3D, rotações, projeções
- **Machine Learning**: Redes neurais, PCA, regressão linear
- **Física/Simulações**: Mecânica, dinâmica de fluidos
- **Processamento de Sinais**: Filtros, transformadas
- **Criptografia**: Operações em corpos finitos

## Desenvolvimento e Testing

### Executar os Testes
```bash
cargo test
```

### Executar o Exemplo Completo
```bash
cargo run
```

### Executar Benchmarks
```bash
cargo bench
```

### Gerar Documentação
```bash
cargo doc --open
```

### Estrutura do Projeto
```
src/
├── lib.rs              # Módulo principal e exports
├── vector.rs           # Implementação de Vector<K>
├── matrix.rs           # Implementação de Matrix<K>
├── linear_combination.rs # Combinações lineares
├── interpolate.rs      # Interpolação linear (LERP)
├── angle_cos.rs        # Cálculo de ângulos
├── cross_product.rs    # Produto vetorial
├── display.rs          # Formatação para impressão
├── errors.rs           # Tipos de erro customizados
└── main.rs             # Exemplos e demonstrações

tests/
├── vector_tests.rs     # Testes de vetores
├── matrix_tests.rs     # Testes de matrizes
├── linear_combination_tests.rs
├── interpolate_tests.rs
├── angle_cos_tests.rs
├── cross_product_tests.rs
└── display_tests.rs
```

### Performance

A biblioteca é otimizada para performance com:
- **Operações in-place** quando possível para evitar alocações
- **Iteradores** para processamento eficiente
- **Métodos funcionais** opcionais que retornam novos objetos
- **Implementações específicas** para matrizes pequenas (determinante)
- **Eliminação Gaussiana otimizada** para operações avançadas

### Compatibilidade

- **Rust Edition**: 2021
- **MSRV**: 1.70.0 (Minimum Supported Rust Version)
- **Dependências**: Biblioteca zero-dependency (apenas std)
- **Plataformas**: Qualquer plataforma suportada pelo Rust

## Contribuição

Este projeto segue as diretrizes da 42 School para o projeto Matrix. Contribuições são bem-vindas, especialmente:

- Otimizações de performance
- Suporte para mais tipos numéricos
- Algoritmos alternativos (ex: decomposição LU, QR)
- Melhoria na documentação e exemplos
- Testes adicionais

## Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo `LICENSE-MIT` para detalhes.

---

**Nota**: Este projeto é uma implementação educacional do currículo da 42 School e serve como base sólida para qualquer aplicação que necessite de operações de álgebra linear em Rust.
