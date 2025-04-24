mod scalar;
use crate::scalar::Scalar;

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            matrix[i][i] = T::one();
        }
        Matrix(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let matrix = Matrix::<f64>::new();
        assert_eq!(matrix, Matrix(vec![vec![0.0]]));
    }

    #[test]
    fn test_zero_matrix() {
        let matrix = Matrix::<f64>::zero(2, 3);
        assert_eq!(matrix, Matrix(vec![
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ]));
    }

    #[test]
    fn test_identity_matrix() {
        let matrix = Matrix::<f64>::identity(3);
        assert_eq!(matrix, Matrix(vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ]));
    }
}
