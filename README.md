Rust implementation of Heer and Stone's C3 library.

The package published on crates.io contains **only Rust sources** (no `c3_*.npy`). Clone this repo or supply your own `.npy` files and load them with [`C3::from_npy_dir`](https://docs.rs/rust_c3/latest/rust_c3/struct.C3.html#method.from_npy_dir), or keep using [`C3::new`](https://docs.rs/rust_c3/latest/rust_c3/struct.C3.html#method.new) when those files live next to the crate source (e.g. local checkout).

## Tests

Correctness tests that do **not** require `.npy` files (small in-memory model):

```text
cargo test
```

Regression tests matching the demo binary on the **full** dataset (needs `src/c3_color.npy`, `src/c3_a.npy`, `src/c3_t.npy` next to this crate):

```text
cargo test --test c3 -- --ignored
```

## Benchmarks

Criterion benchmarks for hot methods (`cargo bench`). Stable names include `tiny_*` (always run) and `full_*` (only if the full `src/c3_*.npy` bundle is present). Compare before/after optimizations using the printed nanoseconds and Criterion’s “change” lines once a baseline exists (`target/criterion`).