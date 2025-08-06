use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum VectorError {
    LinearCombinationDimensionMismatch,
    DimensionMismatch,
}

impl Display for VectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            VectorError::LinearCombinationDimensionMismatch => {
                write!(f, "Número de vetores e coeficientes não coincidem")
            }
            VectorError::DimensionMismatch => {
                write!(f, "Dimensões dos vetores não coincidem")
            }
        }
    }
}

impl Error for VectorError {}

#[derive(Debug)]
pub enum MatrixError {
    RowsLengthMismatch,
    DimensionMismatch,
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MatrixError::RowsLengthMismatch => {
                write!(f, "Tamanho das linhas deve ser sempre igual")
            }
            MatrixError::DimensionMismatch => {
                write!(f, "Dimensões dos vetores não coincidem")
            }
        }
    }
}

impl Error for MatrixError {}
