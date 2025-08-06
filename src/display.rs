use std::fmt::{Display, Formatter, Result};

use crate::{matrix::Matrix, types::Scalar, vector::Vector};

impl<K> Display for Vector<K>
where
    K: Scalar,
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
    K: Scalar,
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
