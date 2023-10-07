use std::ops::Range;

use rand::Rng;
use simd::{dot_product, dot_product_simd, dot_product_rayon};

fn main() {
    divan::main();
}


const SIZES: [usize; 3] = [1000, 100000, 10000000];
const RANGE: Range<f32> = -100f32..100f32;

fn random_f32_vector(size: usize) -> Vec<f32> {
    let mut vec = Vec::with_capacity(size);
    let mut random = rand::thread_rng();
    for _ in 0..size {
        vec.push(random.gen_range(RANGE.clone()));
    }
    vec
}


#[divan::bench(consts = SIZES)]
fn normal<const N: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (random_f32_vector(N), random_f32_vector(N)))
        .bench_values(|(a, b)| dot_product(&a, &b))
}

#[divan::bench(consts = SIZES)]
fn simd<const N: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (random_f32_vector(N), random_f32_vector(N)))
        .bench_values(|(a, b)| dot_product_simd(&a, &b))
}

#[divan::bench(consts = SIZES)]
fn rayon<const N: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (random_f32_vector(N), random_f32_vector(N)))
        .bench_values(|(a, b)| dot_product_rayon(&a, &b))
}
