fn main() {
    divan::main();
}

const N: [usize; 5] = [10, 30, 50, 70, 90];

#[divan::bench(consts = N)]
fn fibonacci_iterative<const N: usize>() -> usize {
    fibonacci::iterative(divan::black_box(N))
}

#[divan::bench(consts = N)]
fn fibonacci_cached<const N: usize>() -> usize {
    let mut cached = fibonacci::Cache::default();
    cached.fibonacci(divan::black_box(N))
}

#[divan::bench(consts = N)]
fn fibonacci_matrix<const N: usize>() -> usize {
    fibonacci::matrix(divan::black_box(N))
}
