pub fn iterative(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }
    c
}

const MAX_FIB: usize = 186;
// const MAX_FIB: usize = 93;
static mut CACHE: [u128; MAX_FIB + 1] = [0; MAX_FIB + 1];
static mut CACHE_LENGTH: usize = 2;

pub fn cached(n: usize) -> u128 {
    if n > MAX_FIB {
        panic!("n is too big");
    }
    unsafe {
        CACHE[0] = 0;
        CACHE[1] = 1;
        for i in CACHE_LENGTH..=n {
            CACHE[i] = CACHE[i - 1] + CACHE[i - 2];
        }
        if CACHE_LENGTH <= n {
            CACHE_LENGTH = n + 1;
        }
        CACHE[n]
    }
}

#[derive(Clone, Copy)]
struct Matrix2x2 {
    data: [u128; 4],
}
impl Matrix2x2 {
    fn mul(&mut self, rhs: Matrix2x2) {
        let a = self.data;
        let b = rhs.data;
        self.data = [
            a[0] * b[0] + a[1] * b[2],
            a[0] * b[1] + a[1] * b[3],
            a[2] * b[0] + a[3] * b[2],
            a[2] * b[1] + a[3] * b[3],
        ];
    }
    const BASE_MATRIX: Matrix2x2 = Matrix2x2 { data: [1, 1, 1, 0] };
    fn pow(&mut self, n: u128) {
        if n <= 1 {
            return;
        }
        self.pow(n / 2);
        self.mul(*self);
        if n % 2 == 1 {
            self.mul(Self::BASE_MATRIX);
        }
    }
}

pub fn matrix(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut fibonacci_matrix = Matrix2x2 { data: [1, 1, 1, 0] };
    fibonacci_matrix.pow(n - 1);
    fibonacci_matrix.data[0]
}

///Note: wrong answer for n >= 76
pub fn formula(n: u128) -> u128 {
    (((1. + 5f64.sqrt()) / 2.).powi(n as i32) / 5f64.sqrt()).round() as u128
}

/// The best method to calculate n-th fibonacci
pub fn hybrid(n: u128) -> u128 {
    // Iterative is efficient in the range of 0 to 10
    if n <= 10 {
        return iterative(n);
    }
    if n <= 75 {
        return formula(n);
    }

    matrix(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIBONACCI: [u128; 10] = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

    #[test]
    fn iterative_test() {
        for (i, fibonacci) in FIBONACCI.iter().enumerate() {
            assert_eq!(iterative(i as u128), *fibonacci);
        }
    }

    #[test]
    fn cached_test() {
        for (i, fibonacci) in FIBONACCI.iter().enumerate() {
            assert_eq!(cached(i), *fibonacci);
        }
    }

    #[test]
    fn matrix_test() {
        for (i, fibonacci) in FIBONACCI.iter().enumerate() {
            assert_eq!(matrix(i as u128), *fibonacci);
        }
    }

    #[test]
    fn formula_test() {
        for (i, fibonacci) in FIBONACCI.iter().enumerate() {
            assert_eq!(formula(i as u128), *fibonacci);
        }
    }

    #[test]
    fn hybrid_test() {
        for (i, fibonacci) in FIBONACCI.iter().enumerate() {
            assert_eq!(hybrid(i as u128), *fibonacci);
        }
    }
}
