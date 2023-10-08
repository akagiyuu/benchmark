pub fn iterative(n: usize) -> usize {
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

pub struct Cache {
    data: Vec<usize>,
}
impl Default for Cache {
    fn default() -> Self {
        Self { data: vec![0, 1] }
    }
}
impl Cache {
    pub fn fibonacci(&mut self, n: usize) -> usize {
        let length = self.data.len();
        for i in length..=n {
            self.data.push(self.data[i - 1] + self.data[i - 2]);
        }

        self.data[n]
    }
}

#[derive(Clone, Copy)]
struct Matrix2x2 {
    data: [usize; 4],
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
    fn pow(&mut self, n: usize) {
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

pub fn matrix(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut fibonacci_matrix = Matrix2x2 { data: [1, 1, 1, 0] };
    fibonacci_matrix.pow(n - 1);
    fibonacci_matrix.data[0]
}

///Note: wrong answer for n >= 76
pub fn formula(n: usize) -> usize {
    (((1. + 5f64.sqrt()) / 2.).powi(n as i32) / 5f64.sqrt()).round() as usize
}

/// The best method to calculate n-th fibonacci
pub fn hybrid(n: usize) -> usize {
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

    const FIBONACCI: [usize; 10] = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

    #[test]
    fn fibonacci_iterative_test() {
        for (i, fibonacci) in FIBONACCI.iter().enumerate() {
            assert_eq!(iterative(i), *fibonacci);
        }
    }

    #[test]
    fn fibonacci_cached_test() {
        let mut cache = Cache::default();
        for (i, fibonacci) in FIBONACCI.iter().enumerate() {
            assert_eq!(cache.fibonacci(i), *fibonacci);
        }
    }

    #[test]
    fn fibonacci_matrix_test() {
        for (i, fibonacci) in FIBONACCI.iter().enumerate() {
            assert_eq!(matrix(i), *fibonacci);
        }
    }

    #[test]
    fn fibonacci_formula_test() {
        for (i, fibonacci) in FIBONACCI.iter().enumerate() {
            assert_eq!(formula(i), *fibonacci);
        }
    }

    #[test]
    fn fibonacci_hybrid_test() {
        for (i, fibonacci) in FIBONACCI.iter().enumerate() {
            assert_eq!(hybrid(i), *fibonacci);
        }
    }
}
