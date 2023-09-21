use rand::Rng;

use crate::number::Number;
use std::ops::{Index, IndexMut, Mul};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FlatMatrix<T: Number<T>, const M: usize, const N: usize>
where
    [(); M * N]:,
{
    pub data: [T; M * N],
}
impl<T: Number<T>, const M: usize, const N: usize> Default for FlatMatrix<T, M, N>
where
    [(); M * N]:,
{
    fn default() -> Self {
        Self {
            data: [T::default(); M * N],
        }
    }
}
impl<T: Number<T>, const M: usize, const N: usize> Index<[usize; 2]> for FlatMatrix<T, M, N>
where
    [(); M * N]:,
{
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[0] * N + index[1]]
    }
}
impl<T: Number<T>, const M: usize, const N: usize> IndexMut<[usize; 2]> for FlatMatrix<T, M, N>
where
    [(); M * N]:,
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.data[index[0] * N + index[1]]
    }
}
impl<T: Number<T>, const M: usize, const N: usize, const P: usize> Mul<FlatMatrix<T, N, P>>
    for FlatMatrix<T, M, N>
where
    [(); M * N]:,
    [(); N * P]:,
    [(); M * P]:,
{
    type Output = FlatMatrix<T, M, P>;

    fn mul(self, rhs: FlatMatrix<T, N, P>) -> Self::Output {
        let mut result = FlatMatrix::default();
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

impl<const M: usize, const N: usize> FlatMatrix<u32, M, N>
where
    [(); M * N]:,
{
    pub fn random() -> Self {
        let mut result = FlatMatrix::default();
        let mut random_thread = rand::thread_rng();
        for i in 0..M {
            for j in 0..N {
                result[[i, j]] = random_thread.gen_range(0..100);
            }
        }
        result
    }
}
