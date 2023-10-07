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
    use super::*;

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
}
