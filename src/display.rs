use std::fmt::{Display, Formatter, Result};

use crate::{
    Complex, InterpolationError, LinearCombinationError, Matrix, MatrixInverseError, Vector,
};

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
        for (j, item) in col_widths.iter_mut().enumerate().take(columns) {
            for i in 0..rows {
                let len = format!("{}", self[i][j]).len();
                if len > *item {
                    *item = len;
                }
            }
        }

        for i in 0..rows {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "[")?;
            for (j, item) in col_widths.iter().enumerate().take(columns) {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:>width$}", self[i][j], width = item)?;
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
                    "Matriz deve ser quadrada para calcular a inversa: encontrada {rows}x{columns}",
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

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let real = self.real();
        let imaginary = self.imaginary();

        if imaginary == 0.0 {
            write!(f, "{real}")
        } else if real == 0.0 {
            if imaginary == 1.0 {
                write!(f, "i")
            } else if imaginary == -1.0 {
                write!(f, "-i")
            } else {
                write!(f, "{imaginary}i")
            }
        } else if imaginary > 0.0 {
            if imaginary == 1.0 {
                write!(f, "{real} + i")
            } else {
                write!(f, "{real} + {imaginary}i")
            }
        } else if imaginary == -1.0 {
            write!(f, "{real} - i",)
        } else {
            write!(f, "{real} - {}i", -imaginary)
        }
    }
}
