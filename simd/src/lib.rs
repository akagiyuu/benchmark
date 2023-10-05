#![feature(array_chunks)]
#![feature(portable_simd)]
#![feature(test)]

use rayon::prelude::*;
use std::simd::{f32x4, SimdFloat};

pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).fold(0., |acc, (a, b)| acc + a * b)
}
pub fn dot_product_simd(a: &[f32], b: &[f32]) -> f32 {
    assert!(a.len() % 4 == 0);
    assert!(a.len() == b.len());
    a.array_chunks::<4>()
        .map(|&chunk| f32x4::from_array(chunk))
        .zip(b.array_chunks::<4>().map(|&chunk| f32x4::from_array(chunk)))
        .fold(f32x4::splat(0.), |acc, (a, b)| acc + a * b)
        .reduce_sum()
}

pub fn dot_product_rayon(a: &[f32], b: &[f32]) -> f32 {
    a.par_iter().zip(b.par_iter()).map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;
    use rand::Rng;
    extern crate test;
    use test::black_box;
    use test::Bencher;

    fn random_f32_vector(size: usize, range: Range<f32>) -> Vec<f32> {
        let mut vec = Vec::with_capacity(size);
        let mut random = rand::thread_rng();
        for _ in 0..size {
            vec.push(random.gen_range(range.clone()));
        }
        vec
    }

    #[test]
    fn dot_product_test() {
        let a = [1., 2., 3., 4.];
        let b = [5., 6., 7., 8.];
        assert_eq!(dot_product(&a, &b), 70.);

        let a = [5., 63., 2., 3., 4., 1., 6., 7.];
        let b = [6., 1., 5., 7., 4., 3., 2., 8.];
        assert_eq!(dot_product(&a, &b), 211.);
    }
    #[test]
    fn dot_product_simd_test() {
        let a = [1., 2., 3., 4.];
        let b = [5., 6., 7., 8.];
        assert_eq!(dot_product_simd(&a, &b), 70.);

        let a = [5., 63., 2., 3., 4., 1., 6., 7.];
        let b = [6., 1., 5., 7., 4., 3., 2., 8.];
        assert_eq!(dot_product_simd(&a, &b), 211.);
    }
    #[test]
    fn dot_product_rayon_test() {
        let a = [1., 2., 3., 4.];
        let b = [5., 6., 7., 8.];
        assert_eq!(dot_product_rayon(&a, &b), 70.);

        let a = [5., 63., 2., 3., 4., 1., 6., 7.];
        let b = [6., 1., 5., 7., 4., 3., 2., 8.];
        assert_eq!(dot_product_rayon(&a, &b), 211.);
    }

    const RANGE: Range<f32> = -100f32..100f32;
    const SIZE: usize = 10000000;

    #[bench]
    fn dot_product_bench(bencher: &mut Bencher) {
        bencher.iter(|| {
            let a = black_box(random_f32_vector(SIZE, RANGE));
            let b = black_box(random_f32_vector(SIZE, RANGE));
            println!("{}", dot_product(&a, &b));
        })
    }

    #[bench]
    fn dot_product_simd_bench(bencher: &mut Bencher) {
        bencher.iter(|| {
            let a = black_box(random_f32_vector(SIZE, RANGE));
            let b = black_box(random_f32_vector(SIZE, RANGE));
            println!("{}", dot_product_simd(&a, &b));
        })
    }

    #[bench]
    fn dot_product_rayon_bench(bencher: &mut Bencher) {
        bencher.iter(|| {
            let a = black_box(random_f32_vector(SIZE, RANGE));
            let b = black_box(random_f32_vector(SIZE, RANGE));
            println!("{}", dot_product_rayon(&a, &b));
        })
    }
}
