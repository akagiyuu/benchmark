#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use matrix::{FlatMatrix, Matrix};
use nalgebra::DMatrix;
use ndarray::{ArrayBase, Dim, OwnedRepr};
use rand::Rng;
fn main() {
    divan::main();
}

const SIZES: [usize; 3] = [10, 100, 500];

#[divan::bench(consts = SIZES)]
fn normal_matrix<const N: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (Matrix::<u32, N, N>::random(), Matrix::<u32, N, N>::random()))
        .bench_values(|(a, b)| a * b)
}

#[divan::bench(consts = SIZES)]
fn flat_matrix<const N: usize>(bencher: divan::Bencher)
where
    [(); N * N]:,
{
    bencher
        .with_inputs(|| {
            (
                FlatMatrix::<u32, N, N>::random(),
                FlatMatrix::<u32, N, N>::random(),
            )
        })
        .bench_values(|(a, b)| a * b);
}

#[divan::bench(consts = SIZES)]
fn nalgebra_matrix<const N: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut random = rand::thread_rng();
            let mut random_matrix = || DMatrix::from_fn(N, N, |_, _| random.gen_range(0..100u32));
            (random_matrix(), random_matrix())
        })
        .bench_values(|(a, b)| a * b);
}

#[divan::bench(consts = SIZES)]
fn ndarray_matrix<const N: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut random = rand::thread_rng();
            let mut random_matrix = || {
                ArrayBase::<OwnedRepr<u32>, Dim<[usize; 2]>>::from_shape_fn((N, N), |_| {
                    random.gen_range(0..100)
                })
            };
            (random_matrix(), random_matrix())
        })
        .bench_values(|(a, b)| a.dot(&b))
}

//             let a = black_box();
//             let b = black_box(DMatrix::from_fn(M, N, |_, _| random.gen_range(0..100u32)));
//
//             let mut random = rand::thread_rng();
//             let a: ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>> =
//                 black_box();
//             let b: ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>> =
//                 black_box(ArrayBase::from_shape_fn((M, N), |_| {
//                     random.gen_range(0..100)
//                 }));
