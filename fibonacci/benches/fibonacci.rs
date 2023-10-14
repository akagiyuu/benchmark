fn main() {
    divan::main();
}

const N: [u128; 5] = [10, 50, 100, 150, 180];

#[divan::bench(consts = N)]
fn fibonacci_iterative<const N: u128>() -> u128 {
    fibonacci::iterative(divan::black_box(N))
}

#[divan::bench(consts = N)]
fn fibonacci_cached<const N: u128>() -> u128 {
    fibonacci::cached(divan::black_box(N as usize))
}

#[divan::bench(consts = N)]
fn fibonacci_matrix<const N: u128>() -> u128 {
    fibonacci::matrix(divan::black_box(N))
}

#[divan::bench(consts = N)]
fn fibonacci_formula<const N: u128>() -> u128 {
    fibonacci::formula(divan::black_box(N))
}

#[divan::bench(consts = N)]
fn fibonacci_hybrid<const N: u128>() -> u128 {
    fibonacci::hybrid(divan::black_box(N))
}
