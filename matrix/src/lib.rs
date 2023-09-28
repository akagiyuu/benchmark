#![feature(test)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod flat_matrix;
pub mod matrix;
pub mod number;

#[cfg(test)]
mod tests {
    extern crate test;
    use test::black_box;
    use test::Bencher;
    use super::*;
    use flat_matrix::FlatMatrix;
    use matrix::Matrix;

    #[test]
    fn normal_matrix_multiply_test() {
        let a = Matrix::<u32, 2, 2> {
            data: [[0, 6], [8, 5]],
        };
        let b = Matrix {
            data: [[5, 0], [2, 3]],
        };
        assert_eq!(
            a * b,
            Matrix {
                data: [[12, 18], [50, 15]]
            }
        );

        let a = Matrix::<u32, 3, 4> {
            data: [[5, 8, 6, 4], [0, 2, 3, 6], [4, 5, 8, 7]],
        };
        let b = Matrix {
            data: [
                [8, 9, 5, 4, 2],
                [6, 4, 5, 8, 7],
                [4, 0, 3, 2, 5],
                [4, 7, 8, 6, 3],
            ],
        };
        assert_eq!(
            a * b,
            Matrix {
                data: [
                    [128, 105, 115, 120, 108],
                    [48, 50, 67, 58, 47],
                    [122, 105, 125, 114, 104],
                ]
            }
        );
    }

    #[test]
    fn flat_matrix_multiply_test() {
        let a = FlatMatrix::<u64, 2, 2> { data: [0, 6, 8, 5] };
        let b = FlatMatrix::<u64, 2, 2> { data: [5, 0, 2, 3] };
        assert_eq!(
            a * b,
            FlatMatrix {
                data: [12, 18, 50, 15]
            }
        );

        let a = FlatMatrix::<u64, 3, 4> {
            data: [5, 8, 6, 4, 0, 2, 3, 6, 4, 5, 8, 7],
        };
        let b = FlatMatrix::<u64, 4, 5> {
            data: [8, 9, 5, 4, 2, 6, 4, 5, 8, 7, 4, 0, 3, 2, 5, 4, 7, 8, 6, 3],
        };
        assert_eq!(
            a * b,
            FlatMatrix {
                data: [128, 105, 115, 120, 108, 48, 50, 67, 58, 47, 122, 105, 125, 114, 104,]
            }
        );
    }

    const N: usize = 400;
    const M: usize = 400;
    const P: usize = 400;

    #[bench]
    fn normal_matrix_multiply_bench(bencher: &mut Bencher) {
        bencher.iter(|| {
            let a = black_box(Matrix::<u32, M, N>::random());
            let b = black_box(Matrix::<u32, N, P>::random());
            a * b
        });
    }

    #[bench]
    fn flat_matrix_multiply_bench(bencher: &mut Bencher) {
        bencher.iter(|| {
            let a = black_box(FlatMatrix::<u32, M, N>::random());
            let b = black_box(FlatMatrix::<u32, N, P>::random());
            a * b
        });
    }
}
