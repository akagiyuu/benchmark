use std::f64::consts::{E, PI};

pub fn iterative(n: usize) -> f64 {
    let mut result = 1.;
    for i in 2..=n {
        result *= i as f64;
    }
    result
}

pub fn recursive(n: usize) -> f64 {
    if n == 0 {
        return 1.;
    }
    (n as f64) * recursive(n - 1)
}

pub fn stirling_factorial_upperbound(n: usize) -> f64 {
    let n = n as f64;
    let lg_n = n.log2();
    let lg_of_factorial =
        (1f64 + PI.log2() + lg_n) / 2f64 + n * (lg_n - E.log2()) + 1f64 / (12f64 * n) * E.log2();
    2f64.powf(lg_of_factorial).round()
}

const MAX_N: usize = 100;
static mut CACHE: [f64; MAX_N + 1] = [0.; MAX_N + 1];
static mut CACHE_LENGTH: usize = 1;

pub fn cached(n: usize) -> f64 {
    if n > MAX_N {
        panic!("n is too large");
    }
    unsafe {
        CACHE[0] = 1.;
        for i in CACHE_LENGTH..=n {
            CACHE[i] = CACHE[i - 1] * i as f64;
        }
        if CACHE_LENGTH <= n {
            CACHE_LENGTH = n + 1;
        }
        CACHE[n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const FACTORIAL: [f64; 21] = [
        1.,
        1.,
        2.,
        6.,
        24.,
        120.,
        720.,
        5040.,
        40320.,
        362880.,
        3628800.,
        39916800.,
        479001600.,
        6227020800.,
        87178291200.,
        1307674368000.,
        20922789888000.,
        355687428096000.,
        6402373705728000.,
        121645100408832000.,
        2432902008176640000.,
    ];
    #[test]
    fn iterative_test() {
        for (i, &factorial) in FACTORIAL.iter().enumerate() {
            assert_eq!(iterative(i), factorial);
        }
    }

    #[test]
    fn recursive_test() {
        for (i, &factorial) in FACTORIAL.iter().enumerate() {
            assert_eq!(recursive(i), factorial);
        }
    }

    #[test]
    fn cached_test() {
        for (i, &factorial) in FACTORIAL.iter().enumerate() {
            assert_eq!(cached(i), factorial);
        }
    }
}
