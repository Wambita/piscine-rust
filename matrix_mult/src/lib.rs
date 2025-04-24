use std::ops::Mul;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() || self.0[0].is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n]).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Mul<Output = T> + std::iter::Sum + Default,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        let rows_a = self.number_of_rows();
        let cols_a = self.number_of_cols();
        let rows_b = rhs.number_of_rows();
        let cols_b = rhs.number_of_cols();

        if cols_a != rows_b {
            return None;
        }

        let mut result = vec![vec![T::default(); cols_b]; rows_a];

        for i in 0..rows_a {
            for j in 0..cols_b {
                result[i][j] = (0..cols_a)
                    .map(|k| self.0[i][k] * rhs.0[k][j])
                    .sum();
            }
        }

        Some(Matrix(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let matrix_a = Matrix(vec![vec![1, 2], vec![3, 4]]);
        let matrix_b = Matrix(vec![vec![5, 6], vec![7, 8]]);
        let result = matrix_a * matrix_b;

        assert_eq!(
            result,
            Some(Matrix(vec![vec![19, 22], vec![43, 50]]))
        );
    }

    #[test]
    fn test_invalid_multiplication() {
        let matrix_a = Matrix(vec![vec![1, 2], vec![3, 4]]);
        let matrix_b = Matrix(vec![vec![5, 6]]);

        let result = matrix_a * matrix_b;
        assert_eq!(result, None);
    }
    #[test]
    fn test_matrix_row() {
        let matrix = Matrix(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let row = matrix.row(1);
        assert_eq!(row, vec![4, 5, 6]);
    }
    #[test]
    fn test_matrix_col() {
        let matrix = Matrix(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let col = matrix.col(1);
        assert_eq!(col, vec![2, 5]);
    }
}
