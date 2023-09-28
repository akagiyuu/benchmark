# Compare 2D matrix and flat matrix

## How to run?

* benchmarks:
  * change N, M, P
  * run `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

* Multiply 2 50x50 matrices  

| benchmark                      | result                      |
| ------------------------------ | --------------------------- |
| flat_matrix_multiply_bench     | 80,637 ns/iter (+/- 4,923)  |
| nalgebra_matrix_multiply_bench | 77,034 ns/iter (+/- 4,780)  |
| ndarray_matrix_multiply_bench  | 117,200 ns/iter (+/- 8,929) |
| normal_matrix_multiply_bench   | 107,167 ns/iter (+/- 6,214) |

* Multiply 2 100x100 matrices

| benchmark                      | result                       |
| ------------------------------ | ---------------------------- |
| flat_matrix_multiply_bench     | 492,863 ns/iter (+/- 22,295) |
| nalgebra_matrix_multiply_bench | 466,559 ns/iter (+/- 30,590) |
| ndarray_matrix_multiply_bench  | 684,711 ns/iter (+/- 40,512) |
| normal_matrix_multiply_bench   | 750,356 ns/iter (+/- 57,111) |

* Multiply 2 400x400 matrices

| benchmark                      | result                             |
| ------------------------------ | ---------------------------------- |
| flat_matrix_multiply_bench     | 46,347,151 ns/iter (+/- 2,591,154) |
| nalgebra_matrix_multiply_bench | 25,439,335 ns/iter (+/- 1,942,604) |
| ndarray_matrix_multiply_bench  | 46,282,423 ns/iter (+/- 3,827,905) |
| normal_matrix_multiply_bench   | 43,389,376 ns/iter (+/- 2,149,290) |
