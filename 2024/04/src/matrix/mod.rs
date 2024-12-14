mod coordinate;
mod error;

use std::ops::Index;

pub use self::coordinate::Coordinate;
pub use self::coordinate::Distance;
pub use self::error::MatrixError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    matrix: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn get(&self, coordinate: impl Into<Coordinate>) -> Option<&T> {
        let coordinate = coordinate.into();
        self.matrix
            .get(coordinate.row)
            .and_then(|row| row.get(coordinate.col))
    }

    pub fn iter_enumerate(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.matrix.iter().enumerate().flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_idx, elem)| (Coordinate::new(row_idx, col_idx), elem))
        })
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Matrix<T> {
    type Error = MatrixError;

    fn try_from(matrix: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let width = matrix
            .first()
            .map(Vec::len)
            .filter(|&width| width > 0)
            .ok_or(MatrixError::EmptyMatrix)?;

        matrix
            .iter()
            .all(|row| row.len() == width)
            .then(|| Matrix { matrix })
            .ok_or(MatrixError::UnequalRowsLength)
    }
}

impl<I, T> Index<I> for Matrix<T>
where
    I: Into<Coordinate>,
{
    type Output = T;

    fn index(&self, coordinate: I) -> &Self::Output {
        let coordinate = coordinate.into();
        &self.matrix[coordinate.row][coordinate.col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tuple() {
        let matrix = Matrix::try_from(vec![vec![1]]).unwrap();

        assert_eq!(Some(&1), matrix.get((0, 0)));
    }

    #[test]
    fn test_get_coordinate() {
        let matrix = Matrix::try_from(vec![vec![1]]).unwrap();

        assert_eq!(Some(&1), matrix.get(Coordinate::new(0, 0)));
    }

    #[test]
    fn test_index_tuple() {
        let matrix = Matrix::try_from(vec![vec![1]]).unwrap();

        assert_eq!(&1, &matrix[(0, 0)]);
    }

    #[test]
    fn test_index_coordinate() {
        let matrix = Matrix::try_from(vec![vec![1]]).unwrap();

        assert_eq!(&1, &matrix[Coordinate::new(0, 0)]);
    }

    #[test]
    #[should_panic]
    fn test_index_out_of_bounds() {
        let matrix = Matrix::try_from(vec![vec![1]]).unwrap();

        let _ = matrix[(3, 3)];
    }

    #[test]
    fn test_iter_enumerate() {
        let matrix = Matrix::try_from(vec![vec![1, 2], vec![3, 4]]).unwrap();

        assert_eq!(
            vec![
                (Coordinate::new(0, 0), &1),
                (Coordinate::new(0, 1), &2),
                (Coordinate::new(1, 0), &3),
                (Coordinate::new(1, 1), &4),
            ],
            matrix.iter_enumerate().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_invalid_matrix() {
        assert_eq!(
            Err(MatrixError::UnequalRowsLength),
            Matrix::try_from(vec![vec![1, 2], vec![3]])
        );
    }

    #[test]
    fn test_matrix_from_empty_vector() {
        assert_eq!(
            Err(MatrixError::EmptyMatrix),
            Matrix::<char>::try_from(vec![])
        );
    }

    #[test]
    fn test_matrix_from_empty_row() {
        assert_eq!(
            Err(MatrixError::EmptyMatrix),
            Matrix::<char>::try_from(vec![vec![]])
        );
    }
}
