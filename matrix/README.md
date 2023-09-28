# Compare 2D matrix and flat matrix

## How to run?

* benchmarks:
  * change N, M, P
  * run `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

* Multiply 2 50x50 matrices

flat_matrix_multiply_bench          78,602 ns/iter (+/- 5,380)
nalgebra_matrix_multiply_bench      76,946 ns/iter (+/- 4,105)
ndarray_matrix_multiply_bench       46,254 ns/iter (+/- 3,000)
normal_matrix_multiply_bench       106,689 ns/iter (+/- 4,209)

* Multiply 2 100x100 matrices

flat_matrix_multiply_bench         500,515 ns/iter (+/- 40,693)
nalgebra_matrix_multiply_bench     467,279 ns/iter (+/- 23,016)
ndarray_matrix_multiply_bench      186,015 ns/iter (+/- 12,312)
normal_matrix_multiply_bench       745,341 ns/iter (+/- 49,542)

* Multiply 2 400x400 matrices

flat_matrix_multiply_bench      45,775,112 ns/iter (+/- 876,771)
nalgebra_matrix_multiply_bench  25,416,333 ns/iter (+/- 739,142)
ndarray_matrix_multiply_bench    3,466,733 ns/iter (+/- 161,325)
normal_matrix_multiply_bench    43,041,195 ns/iter (+/- 955,663)
