# Compare multiple implementations of matrix

## How to run?

* benchmarks: `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

* Multiply

``` bash
multiply            fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ flat_matrix                    │               │               │               │         │
│  ├─ 10            379 ns        │ 404.3 ns      │ 384.3 ns      │ 385.9 ns      │ 100     │ 800
│  ├─ 100           345.1 µs      │ 503.8 µs      │ 366.8 µs      │ 372.8 µs      │ 100     │ 100
│  ╰─ 500           85.61 ms      │ 93.17 ms      │ 86.9 ms       │ 87.28 ms      │ 100     │ 100
├─ nalgebra_matrix                │               │               │               │         │
│  ├─ 10            431.7 ns      │ 4.08 µs       │ 442.8 ns      │ 486.2 ns      │ 100     │ 400
│  ├─ 100           307.4 µs      │ 441.6 µs      │ 307.5 µs      │ 324.9 µs      │ 100     │ 100
│  ╰─ 500           39.87 ms      │ 43.83 ms      │ 40.73 ms      │ 40.87 ms      │ 100     │ 100
├─ ndarray_matrix                 │               │               │               │         │
│  ├─ 10            899.2 ns      │ 8.452 µs      │ 904.4 ns      │ 1.252 µs      │ 100     │ 200
│  ├─ 100           458.7 µs      │ 636.5 µs      │ 461.7 µs      │ 481.1 µs      │ 100     │ 100
│  ╰─ 500           90.05 ms      │ 98.63 ms      │ 91.27 ms      │ 91.65 ms      │ 100     │ 100
╰─ normal_matrix                  │               │               │               │         │
   ├─ 10            311.7 ns      │ 346.2 ns      │ 315.9 ns      │ 317.8 ns      │ 100     │ 800
   ├─ 100           568.4 µs      │ 856.8 µs      │ 568.6 µs      │ 595.5 µs      │ 100     │ 100
   ╰─ 500           73.2 ms       │ 82.53 ms      │ 74.33 ms      │ 74.72 ms      │ 100     │ 100
```
