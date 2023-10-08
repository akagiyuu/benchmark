# Compare multiple ways to calculate nth Fibonacci

## How to run?

* benchmarks: `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

* Multiply

``` bash
fibonacci               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ fibonacci_cached                   │               │               │               │         │
│  ├─ 10                137.7 ns      │ 182 ns        │ 141.1 ns      │ 143.2 ns      │ 100     │ 1600
│  ├─ 30                234.3 ns      │ 2.453 µs      │ 252 ns        │ 307.5 ns      │ 100     │ 800
│  ├─ 50                303.7 ns      │ 2.487 µs      │ 315.5 ns      │ 380 ns        │ 100     │ 800
│  ├─ 70                377.4 ns      │ 6.683 µs      │ 396.8 ns      │ 533.9 ns      │ 100     │ 400
│  ╰─ 90                425.7 ns      │ 4.841 µs      │ 505.9 ns      │ 582.8 ns      │ 100     │ 400
├─ fibonacci_iterative                │               │               │               │         │
│  ├─ 10                9.241 ns      │ 76.9 ns       │ 9.436 ns      │ 11.55 ns      │ 100     │ 25600
│  ├─ 30                21.11 ns      │ 156.9 ns      │ 21.5 ns       │ 26.77 ns      │ 100     │ 12800
│  ├─ 50                34.4 ns       │ 308.5 ns      │ 35.31 ns      │ 45.77 ns      │ 100     │ 6400
│  ├─ 70                47.82 ns      │ 307 ns        │ 48.13 ns      │ 56.03 ns      │ 100     │ 6400
│  ╰─ 90                61.17 ns      │ 595.3 ns      │ 61.63 ns      │ 72.24 ns      │ 100     │ 3200
╰─ fibonacci_matrix                   │               │               │               │         │
   ├─ 10                12.63 ns      │ 146 ns        │ 12.78 ns      │ 17.9 ns       │ 100     │ 12800
   ├─ 30                17.75 ns      │ 137.7 ns      │ 18.28 ns      │ 19.5 ns       │ 100     │ 12800
   ├─ 50                20.19 ns      │ 258.4 ns      │ 21.61 ns      │ 25.79 ns      │ 100     │ 12800
   ├─ 70                24.54 ns      │ 172.6 ns      │ 24.81 ns      │ 30.6 ns       │ 100     │ 12800
   ╰─ 90                24.93 ns      │ 289.2 ns      │ 25.49 ns      │ 36.08 ns      │ 100     │ 6400
```
