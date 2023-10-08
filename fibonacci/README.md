# Compare multiple ways to calculate nth Fibonacci

## How to run?

* benchmarks: `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

``` bash
fibonacci               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ fibonacci_cached                   │               │               │               │         │
│  ├─ 10                134.1 ns      │ 190.2 ns      │ 137.7 ns      │ 139.4 ns      │ 100     │ 1600
│  ├─ 30                224.2 ns      │ 2.387 µs      │ 234 ns        │ 338.6 ns      │ 100     │ 800
│  ├─ 50                283.4 ns      │ 2.661 µs      │ 299.8 ns      │ 510.7 ns      │ 100     │ 800
│  ├─ 70                401.7 ns      │ 17.62 µs      │ 520.2 ns      │ 945 ns        │ 100     │ 100
│  ╰─ 90                434.7 ns      │ 8.725 µs      │ 550.7 ns      │ 869.2 ns      │ 100     │ 200
├─ fibonacci_formula                  │               │               │               │         │
│  ├─ 10                10.01 ns      │ 404.2 ns      │ 10.47 ns      │ 14.44 ns      │ 100     │ 25600
│  ├─ 30                11.3 ns       │ 12.52 ns      │ 11.56 ns      │ 11.61 ns      │ 100     │ 25600
│  ├─ 50                11.57 ns      │ 130.3 ns      │ 12.3 ns       │ 13.29 ns      │ 100     │ 12800
│  ├─ 70                12.07 ns      │ 148.1 ns      │ 12.75 ns      │ 16.57 ns      │ 100     │ 12800
│  ╰─ 90                11.98 ns      │ 142.5 ns      │ 13.01 ns      │ 16.68 ns      │ 100     │ 12800
├─ fibonacci_hybrid                   │               │               │               │         │
│  ├─ 10                10.4 ns       │ 76.27 ns      │ 10.75 ns      │ 13.44 ns      │ 100     │ 25600
│  ├─ 30                11.37 ns      │ 149.1 ns      │ 13.23 ns      │ 15.89 ns      │ 100     │ 12800
│  ├─ 50                13.11 ns      │ 539.6 ns      │ 13.42 ns      │ 24.49 ns      │ 100     │ 3200
│  ├─ 70                12.57 ns      │ 130.7 ns      │ 14.12 ns      │ 15.37 ns      │ 100     │ 12800
│  ╰─ 90                24.94 ns      │ 278.9 ns      │ 25.19 ns      │ 30.36 ns      │ 100     │ 6400
├─ fibonacci_iterative                │               │               │               │         │
│  ├─ 10                9.515 ns      │ 13.25 ns      │ 10.11 ns      │ 10.28 ns      │ 100     │ 25600
│  ├─ 30                27.57 ns      │ 34.61 ns      │ 28.04 ns      │ 28.57 ns      │ 100     │ 6400
│  ├─ 50                45.47 ns      │ 317.9 ns      │ 47.09 ns      │ 59.28 ns      │ 100     │ 6400
│  ├─ 70                64.89 ns      │ 81.99 ns      │ 65.14 ns      │ 65.75 ns      │ 100     │ 3200
│  ╰─ 90                83.11 ns      │ 90.89 ns      │ 83.46 ns      │ 84.48 ns      │ 100     │ 3200
╰─ fibonacci_matrix                   │               │               │               │         │
   ├─ 10                13.09 ns      │ 14.76 ns      │ 13.32 ns      │ 13.38 ns      │ 100     │ 12800
   ├─ 30                18.08 ns      │ 19.66 ns      │ 18.3 ns       │ 18.36 ns      │ 100     │ 12800
   ├─ 50                20.76 ns      │ 22.38 ns      │ 20.99 ns      │ 21.09 ns      │ 100     │ 12800
   ├─ 70                25.38 ns      │ 28.19 ns      │ 25.6 ns       │ 25.7 ns       │ 100     │ 6400
   ╰─ 90                25.67 ns      │ 259.8 ns      │ 26.7 ns       │ 28.92 ns      │ 100     │ 6400
```
