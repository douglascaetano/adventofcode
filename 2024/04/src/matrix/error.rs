use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MatrixError {
    UnequalRowsLength,
    EmptyMatrix,
}

impl Error for MatrixError {}

impl Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::UnequalRowsLength => write!(f, "Length of rows are not equal"),
            MatrixError::EmptyMatrix => write!(f, "Matrix is empty"),
        }
    }
}
