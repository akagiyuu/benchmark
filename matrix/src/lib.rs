#![feature(test)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod flat_matrix;
pub mod matrix;
pub mod number;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use flat_matrix::FlatMatrix;
    use matrix::Matrix;
    use nalgebra::DMatrix;
    use ndarray::ArrayBase;
    use ndarray::Dim;
    use ndarray::OwnedRepr;
    use rand::Rng;
    use test::black_box;
    use test::Bencher;

    const N: usize = 400;
    const M: usize = 400;
    const P: usize = 400;

    #[bench]
    fn normal_matrix_multiply_bench(bencher: &mut Bencher) {
        bencher.iter(|| {
            let a = black_box(Matrix::<u32, M, N>::random());
            let b = black_box(Matrix::<u32, N, P>::random());
            a * b
        })
    }

    #[bench]
    fn flat_matrix_multiply_bench(bencher: &mut Bencher) {
        bencher.iter(|| {
            let a = black_box(FlatMatrix::<u32, M, N>::random());
            let b = black_box(FlatMatrix::<u32, N, P>::random());
            a * b
        })
    }

    #[bench]
    fn nalgebra_matrix_multiply_bench(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut random = rand::thread_rng();
            let a = black_box(DMatrix::from_fn(M, N, |_, _| random.gen_range(0..100u32)));
            let b = black_box(DMatrix::from_fn(M, N, |_, _| random.gen_range(0..100u32)));
            a * b
        })
    }

    #[bench]
    fn ndarray_matrix_multiply_bench(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut random = rand::thread_rng();
            let a: ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>> =
                black_box(ArrayBase::from_shape_fn((M, N), |_| {
                    random.gen_range(0..100)
                }));
            let b: ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>> =
                black_box(ArrayBase::from_shape_fn((M, N), |_| {
                    random.gen_range(0..100)
                }));
            a * b
        });
    }
}
