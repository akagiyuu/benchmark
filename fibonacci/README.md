# Compare multiple ways to calculate nth Fibonacci

## How to run?

* benchmarks: `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

``` bash
fibonacci               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ fibonacci_cached                   │               │               │               │         │
│  ├─ 10                3.282 ns      │ 3.395 ns      │ 3.297 ns      │ 3.301 ns      │ 100     │ 51200
│  ├─ 50                3.276 ns      │ 35.9 ns       │ 3.293 ns      │ 3.939 ns      │ 100     │ 51200
│  ├─ 100               3.286 ns      │ 34.23 ns      │ 3.297 ns      │ 3.928 ns      │ 100     │ 51200
│  ├─ 150               2.985 ns      │ 48.05 ns      │ 3.024 ns      │ 5.083 ns      │ 100     │ 51200
│  ╰─ 180               3.176 ns      │ 33.05 ns      │ 3.287 ns      │ 3.592 ns      │ 100     │ 51200
├─ fibonacci_formula                  │               │               │               │         │
│  ├─ 10                11.89 ns      │ 80.3 ns       │ 12.29 ns      │ 16.01 ns      │ 100     │ 25600
│  ├─ 50                13.76 ns      │ 151.4 ns      │ 14.22 ns      │ 23.1 ns       │ 100     │ 12800
│  ├─ 100               14.92 ns      │ 152.7 ns      │ 15.65 ns      │ 23.3 ns       │ 100     │ 12800
│  ├─ 150               16.25 ns      │ 148.3 ns      │ 17.28 ns      │ 21.05 ns      │ 100     │ 12800
│  ╰─ 180               16.07 ns      │ 140.2 ns      │ 16.39 ns      │ 18.09 ns      │ 100     │ 12800
├─ fibonacci_hybrid                   │               │               │               │         │
│  ├─ 10                15.47 ns      │ 140.4 ns      │ 16.37 ns      │ 17.81 ns      │ 100     │ 12800
│  ├─ 50                15.45 ns      │ 143.7 ns      │ 16.02 ns      │ 18.85 ns      │ 100     │ 12800
│  ├─ 100               55.14 ns      │ 550 ns        │ 57.58 ns      │ 62.81 ns      │ 100     │ 3200
│  ├─ 150               62.8 ns       │ 624.6 ns      │ 63.47 ns      │ 91.48 ns      │ 100     │ 3200
│  ╰─ 180               63.89 ns      │ 711.1 ns      │ 64.58 ns      │ 86.86 ns      │ 100     │ 3200
├─ fibonacci_iterative                │               │               │               │         │
│  ├─ 10                14.58 ns      │ 15.14 ns      │ 14.64 ns      │ 14.66 ns      │ 100     │ 12800
│  ├─ 50                71.08 ns      │ 73.08 ns      │ 71.39 ns      │ 71.41 ns      │ 100     │ 3200
│  ├─ 100               149.4 ns      │ 151.5 ns      │ 150 ns        │ 150 ns        │ 100     │ 1600
│  ├─ 150               219.7 ns      │ 227.9 ns      │ 220.3 ns      │ 220.6 ns      │ 100     │ 800
│  ╰─ 180               262.4 ns      │ 2.797 µs      │ 263.5 ns      │ 351.4 ns      │ 100     │ 800
╰─ fibonacci_matrix                   │               │               │               │         │
   ├─ 10                27.36 ns      │ 73.82 ns      │ 27.61 ns      │ 28.11 ns      │ 100     │ 6400
   ├─ 50                45.21 ns      │ 320.3 ns      │ 45.77 ns      │ 51.35 ns      │ 100     │ 6400
   ├─ 100               54.77 ns      │ 62.61 ns      │ 57.07 ns      │ 57.3 ns       │ 100     │ 3200
   ├─ 150               61.68 ns      │ 68.39 ns      │ 62.74 ns      │ 63.61 ns      │ 100     │ 3200
   ╰─ 180               62.8 ns       │ 578.1 ns      │ 65.25 ns      │ 71.19 ns      │ 100     │ 3200
```
