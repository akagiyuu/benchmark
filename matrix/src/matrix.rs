use rand::Rng;

use crate::number::Number;
use std::ops::{Index, IndexMut, Mul};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Matrix<T: Number<T>, const M: usize, const N: usize> {
    pub data: [[T; N]; M],
}
impl<T: Number<T>, const M: usize, const N: usize> Default for Matrix<T, M, N> {
    fn default() -> Self {
        Self {
            data: [[T::default(); N]; M],
        }
    }
}
impl<T: Number<T>, const M: usize, const N: usize> Index<[usize; 2]> for Matrix<T, M, N> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[0]][index[1]]
    }
}
impl<T: Number<T>, const M: usize, const N: usize> IndexMut<[usize; 2]> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.data[index[0]][index[1]]
    }
}
impl<T: Number<T>, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>>
    for Matrix<T, M, N>
{
    type Output = Matrix<T, M, P>;

    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        let mut result = Matrix::default();
        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    result[[i, j]] += self[[i, k]] * rhs[[k, j]];
                }
            }
        }
        result
    }
}

impl<T: Number<T>, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut result = Matrix::<T, N, M>::default();
        for (i, row) in self.data.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                result.data[j][i] = *cell;
            }
        }
        result
    }
    pub fn mul_transpose<const P: usize>(self, rhs: Matrix<T, N, P>) -> Matrix<T, M, P> {
        let mut result = Matrix::default();
        let rhs = rhs.transpose();
        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    result[[i, j]] += self[[i, k]] * rhs[[j, k]];
                }
            }
        }
        result
    }
}
impl<const M: usize, const N: usize> Matrix<u32, M, N> {
    pub fn random() -> Self {
        let mut result = Matrix::default();
        let mut random_thread = rand::thread_rng();
        for i in 0..M {
            for j in 0..N {
                result[[i, j]] = random_thread.gen_range(0..100);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn multiply() {
        let a = Matrix::<u32, 2, 2> {
            data: [[0, 6], [8, 5]],
        };
        let b = Matrix {
            data: [[5, 0], [2, 3]],
        };
        assert_eq!(
            a * b,
            Matrix {
                data: [[12, 18], [50, 15]]
            }
        );

        let a = Matrix::<u32, 3, 4> {
            data: [[5, 8, 6, 4], [0, 2, 3, 6], [4, 5, 8, 7]],
        };
        let b = Matrix {
            data: [
                [8, 9, 5, 4, 2],
                [6, 4, 5, 8, 7],
                [4, 0, 3, 2, 5],
                [4, 7, 8, 6, 3],
            ],
        };
        assert_eq!(
            a * b,
            Matrix {
                data: [
                    [128, 105, 115, 120, 108],
                    [48, 50, 67, 58, 47],
                    [122, 105, 125, 114, 104],
                ]
            }
        );
    }
}
