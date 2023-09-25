# Compare normal dot product with SIMD dot product

## How to run?

* benchmarks: `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

```bash
$ cargo +nightly bench

running 4 tests
test tests::dot_product_simd_test ... ignored
test tests::dot_product_test ... ignored
test tests::dot_product_bench      ... bench:  13,636,042 ns/iter (+/- 1,066,865)
test tests::dot_product_simd_bench ... bench:  12,686,951 ns/iter (+/- 1,056,809)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured; 0 filtered out; finished in 7.99s
```
