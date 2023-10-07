# Compare SIMD with other traditional methods

## How to run?

* benchmarks: `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

* Dot product

```bash
dot_product     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ normal                     │               │               │               │         │
│  ├─ 1000      1.2 µs        │ 18.64 µs      │ 1.259 µs      │ 2.049 µs      │ 100     │ 100
│  ├─ 100000    144.2 µs      │ 412.5 µs      │ 151.1 µs      │ 167.2 µs      │ 100     │ 100
│  ╰─ 10000000  12.6 ms       │ 14.98 ms      │ 13.41 ms      │ 13.46 ms      │ 100     │ 100
├─ rayon                      │               │               │               │         │
│  ├─ 1000      14.97 µs      │ 263.7 µs      │ 15.05 µs      │ 20.38 µs      │ 100     │ 100
│  ├─ 100000    156.1 µs      │ 389.5 µs      │ 208.9 µs      │ 225.2 µs      │ 100     │ 100
│  ╰─ 10000000  12.57 ms      │ 17.45 ms      │ 13.34 ms      │ 13.47 ms      │ 100     │ 100
╰─ simd                       │               │               │               │         │
   ├─ 1000      322.4 ns      │ 386.3 ns      │ 326.9 ns      │ 328.1 ns      │ 100     │ 800
   ├─ 100000    61.9 µs       │ 190.5 µs      │ 68.83 µs      │ 75.91 µs      │ 100     │ 100
   ╰─ 10000000  4.379 ms      │ 6.802 ms      │ 4.712 ms      │ 4.901 ms      │ 100     │ 100
```
