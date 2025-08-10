use std::fmt::{Display, Formatter, Result};

use crate::{InterpolationError, LinearCombinationError, Matrix, MatrixInverseError, Vector};

impl<K> Display for Vector<K>
where
    K: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[")?;
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{item}")?;
        }
        write!(f, "]")
    }
}

impl<K> Display for Matrix<K>
where
    K: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let (rows, columns) = self.shape();

        let mut col_widths = vec![0; columns];
        for j in 0..columns {
            for i in 0..rows {
                let len = format!("{}", self[i][j]).len();
                if len > col_widths[j] {
                    col_widths[j] = len;
                }
            }
        }

        for i in 0..rows {
            if i > 0 {
                write!(f, "\n")?;
            }
            write!(f, "[")?;
            for j in 0..columns {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:>width$}", self[i][j], width = col_widths[j])?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

impl Display for LinearCombinationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LinearCombinationError::VectorsDimensionMismatch {
                expected_len,
                founded_len,
            } => {
                write!(
                    f,
                    "Os vetores possuem tamanhos diferentes: esperado {expected_len}, encontrado {founded_len}"
                )
            }
        }
    }
}

impl Display for MatrixInverseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MatrixInverseError::NotSquare { rows, columns } => {
                write!(
                    f,
                    "Matriz deve ser quadrada para calcular a inversa: encontrada {}x{}",
                    rows, columns
                )
            }
            MatrixInverseError::Singular => {
                write!(
                    f,
                    "Matriz é singular (determinante zero) e não possui inversa"
                )
            }
        }
    }
}

impl Display for InterpolationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            InterpolationError::InvalidParameterT { t } => {
                write!(f, "Parâmetro t ({t}) deve estar no intervalo [0, 1]")
            }
        }
    }
}
