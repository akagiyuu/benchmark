# Compare 2D matrix and flat matrix

## How to run?

* benchmarks:
  * change N, M, P
  * run `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

* Multiply 2 50x50 matrices

```bash
$ cargo +nightly bench

running 4 tests
test flat_matrix_multiply_test ... ignored
test normal_matrix_multiply_test ... ignored
test flat_matrix_multiply   ... bench:      80,546 ns/iter (+/- 3,847)
test normal_matrix_multiply ... bench:     108,868 ns/iter (+/- 13,830)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured; 0 filtered out; finished in 10.24s
```

* Multiply 2 100x100 matrices

```bash
$ cargo +nightly bench

running 4 tests
test flat_matrix_multiply_test ... ignored
test normal_matrix_multiply_test ... ignored
test flat_matrix_multiply   ... bench:     488,746 ns/iter (+/- 27,970)
test normal_matrix_multiply ... bench:     750,972 ns/iter (+/- 67,627)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured; 0 filtered out; finished in 7.95s
```

* Multiply 2 400x400 matrices

```bash
$ cargo +nightly bench

running 4 tests
test flat_matrix_multiply_test ... ignored
test normal_matrix_multiply_test ... ignored
test flat_matrix_multiply   ... bench:  46,017,882 ns/iter (+/- 1,314,581)
test normal_matrix_multiply ... bench:  43,160,221 ns/iter (+/- 1,104,903)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured; 0 filtered out; finished in 26.81s
```
