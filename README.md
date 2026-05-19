Rust implementation of Heer and Stone's C3 library.

The crates.io package includes **`src/c3_color.npy`**, **`src/c3_a.npy`**, and **`src/c3_t.npy`**, so [`C3::new`](https://docs.rs/rust_c3/latest/rust_c3/struct.C3.html#method.new) works when you depend on this crate (files resolve via that crate’s manifest directory). To load custom paths, use [`C3::from_npy_dir`](https://docs.rs/rust_c3/latest/rust_c3/struct.C3.html#method.from_npy_dir).

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