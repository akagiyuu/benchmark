# Compare SIMD with other traditional methods

## How to run?

* benchmarks: `cargo +nightly bench`

* tests: `cargo +nightly test`

## Result

* Dot product

```bash
dot_product     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ normal                     │               │               │               │         │
│  ├─ 1000      1.188 µs      │ 1.468 µs      │ 1.191 µs      │ 1.199 µs      │ 100     │ 100
│  │            841.7 Mitem/s │ 681 Mitem/s   │ 839 Mitem/s   │ 833.4 Mitem/s │         │
│  ├─ 100000    144.1 µs      │ 312.2 µs      │ 148.6 µs      │ 162.7 µs      │ 100     │ 100
│  │            693.5 Mitem/s │ 320.2 Mitem/s │ 672.6 Mitem/s │ 614.2 Mitem/s │         │
│  ╰─ 10000000  12.67 ms      │ 14.61 ms      │ 13.33 ms      │ 13.4 ms       │ 100     │ 100
│               789.2 Mitem/s │ 684.1 Mitem/s │ 749.7 Mitem/s │ 746 Mitem/s   │         │
├─ rayon                      │               │               │               │         │
│  ├─ 1000      15.86 µs      │ 226.7 µs      │ 16.02 µs      │ 19.49 µs      │ 100     │ 100
│  │            63.03 Mitem/s │ 4.41 Mitem/s  │ 62.41 Mitem/s │ 51.3 Mitem/s  │         │
│  ├─ 100000    160.2 µs      │ 364 µs        │ 208.3 µs      │ 216.7 µs      │ 100     │ 100
│  │            623.9 Mitem/s │ 274.7 Mitem/s │ 479.8 Mitem/s │ 461.4 Mitem/s │         │
│  ╰─ 10000000  12.56 ms      │ 14.78 ms      │ 13.4 ms       │ 13.39 ms      │ 100     │ 100
│               795.8 Mitem/s │ 676.4 Mitem/s │ 745.8 Mitem/s │ 746.3 Mitem/s │         │
╰─ simd                       │               │               │               │         │
   ├─ 1000      332.8 ns      │ 649.4 ns      │ 345.4 ns      │ 362.8 ns      │ 100     │ 400
   │            3.003 Gitem/s │ 1.539 Gitem/s │ 2.894 Gitem/s │ 2.756 Gitem/s │         │
   ├─ 100000    61.94 µs      │ 303.5 µs      │ 66.16 µs      │ 81.56 µs      │ 100     │ 100
   │            1.614 Gitem/s │ 329.4 Mitem/s │ 1.511 Gitem/s │ 1.226 Gitem/s │         │
   ╰─ 10000000  4.374 ms      │ 5.973 ms      │ 4.638 ms      │ 4.804 ms      │ 100     │ 100
                2.285 Gitem/s │ 1.674 Gitem/s │ 2.155 Gitem/s │ 2.081 Gitem/s │         │
```
