use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum LinearCombinationError {
    VectorsDimensionMismatch {
        expected_len: usize,
        founded_len: usize,
    },
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

impl Error for LinearCombinationError {}

#[derive(Debug)]
pub enum InterpolationError {
    InvalidParameterT { t: f32 },
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

impl Error for InterpolationError {}
