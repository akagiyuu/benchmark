# Compare SIMD dot product with other implementations

## How to run?

* benchmarks: `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

* Dot product of 2 10000000 elements vectors

| benchmark               | result                              |
| ----------------------- | ----------------------------------- |
| dot_product_bench       | 152,594,464 ns/iter (+/- 1,940,396) |
| dot_product_rayon_bench | 145,130,206 ns/iter (+/- 6,536,343) |
| dot_product_simd_bench  | 144,510,480 ns/iter (+/- 1,284,809) |

* Dot product of 2 100000 elements vectors

| benchmark               | result                          |
| ----------------------- | ------------------------------- |
| dot_product_bench       | 1,413,525 ns/iter (+/- 64,231)  |
| dot_product_rayon_bench | 1,548,394 ns/iter (+/- 117,302) |
| dot_product_simd_bench  | 1,329,162 ns/iter (+/- 49,746)  |

* Dot product of 2 1000 elements vectors

| benchmark               | result                     |
| ----------------------- | -------------------------- |
| dot_product_bench       | 14,581 ns/iter (+/- 848)   |
| dot_product_rayon_bench | 32,525 ns/iter (+/- 2,332) |
| dot_product_simd_bench  | 13,660 ns/iter (+/- 378)   |
