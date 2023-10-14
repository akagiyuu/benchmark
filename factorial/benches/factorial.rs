use divan::black_box;

fn main() {
    divan::main();
}

const N: [usize; 4] = [10, 20, 50, 100];

#[divan::bench(consts = N)]
fn factorial_iterative<const N: usize>() -> f64 {
    factorial::iterative(black_box(N))
}

#[divan::bench(consts = N)]
fn factorial_recursive<const N: usize>() -> f64 {
    factorial::recursive(black_box(N))
}

#[divan::bench(consts = N)]
fn factorial_stirling<const N: usize>() -> f64 {
    factorial::stirling_factorial_upperbound(black_box(N))
}

#[divan::bench(consts = N)]
fn factorial_cached<const N: usize>() -> f64 {
    factorial::cached(black_box(N))
}
