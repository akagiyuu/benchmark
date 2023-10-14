# Compare multiple ways to calculate factorial

## How to run?

* benchmarks: `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

``` bash
factorial               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ factorial_cached                   │               │               │               │         │
│  ├─ 10                2.966 ns      │ 2.985 ns      │ 2.977 ns      │ 2.977 ns      │ 100     │ 51200
│  ├─ 20                3.259 ns      │ 3.278 ns      │ 3.269 ns      │ 3.269 ns      │ 100     │ 51200
│  ├─ 50                2.968 ns      │ 33.47 ns      │ 2.977 ns      │ 3.285 ns      │ 100     │ 51200
│  ╰─ 100               3.156 ns      │ 33 ns         │ 3.172 ns      │ 3.506 ns      │ 100     │ 51200
├─ factorial_iterative                │               │               │               │         │
│  ├─ 10                10.44 ns      │ 141.2 ns      │ 10.5 ns       │ 14.3 ns       │ 100     │ 12800
│  ├─ 20                20.09 ns      │ 282.1 ns      │ 20.36 ns      │ 30.6 ns       │ 100     │ 6400
│  ├─ 50                52.59 ns      │ 548.7 ns      │ 54.33 ns      │ 88.12 ns      │ 100     │ 3200
│  ╰─ 100               113.6 ns      │ 1.2 µs        │ 119.5 ns      │ 151.3 ns      │ 100     │ 1600
├─ factorial_recursive                │               │               │               │         │
│  ├─ 10                25.59 ns      │ 272.2 ns      │ 25.75 ns      │ 30.69 ns      │ 100     │ 6400
│  ├─ 20                47.74 ns      │ 542.6 ns      │ 47.91 ns      │ 52.98 ns      │ 100     │ 3200
│  ├─ 50                117.7 ns      │ 1.26 µs       │ 118.2 ns      │ 161.3 ns      │ 100     │ 1600
│  ╰─ 100               237.2 ns      │ 2.341 µs      │ 238.9 ns      │ 340.4 ns      │ 100     │ 800
╰─ factorial_stirling                 │               │               │               │         │
   ├─ 10                62.04 ns      │ 5.284 µs      │ 64.55 ns      │ 116.4 ns      │ 100     │ 100
   ├─ 20                23.52 ns      │ 283.7 ns      │ 24.33 ns      │ 34.37 ns      │ 100     │ 6400
   ├─ 50                23.51 ns      │ 286.8 ns      │ 25.08 ns      │ 45.46 ns      │ 100     │ 6400
   ╰─ 100               25.55 ns      │ 287.3 ns      │ 26.79 ns      │ 44.79 ns      │ 100     │ 6400
```
