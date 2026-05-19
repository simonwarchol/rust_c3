//! Criterion benchmarks for hot paths.
//!
//! ```text
//! cargo bench
//! ```
//!
//! Optional (when full `src/c3_*.npy` bundle exists): runs extra functions prefixed with `full_*`.

use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::{array, Array1, Array2};
use rust_c3::{get_c3_terms, C3};

fn tiny_c3() -> C3 {
    let w = get_c3_terms().len() as i64;
    let color = Array2::from_shape_vec((3, 3), vec![0, 0, 0, 100, 0, 0, 0, 100, 0]).unwrap();
    let a = Array1::zeros(3);
    let t_vec = array![0i64, 10, 1, 30, w, 5, w + 2, 5];
    C3::from_raw_parts(color, a, t_vec)
}

fn bench_tiny_entropy(c: &mut Criterion) {
    let model = tiny_c3();
    c.bench_function("tiny_color_entropy", |b| {
        b.iter(|| black_box(model.color_entropy(black_box(0))))
    });
}

fn bench_tiny_related_terms(c: &mut Criterion) {
    let model = tiny_c3();
    c.bench_function("tiny_color_related_terms_top10", |b| {
        b.iter(|| {
            black_box(model.color_related_terms(
                black_box(0),
                Some(10),
                None,
                None,
            ))
        })
    });
}

fn bench_tiny_cosine(c: &mut Criterion) {
    let model = tiny_c3();
    c.bench_function("tiny_color_cosine", |b| {
        b.iter(|| black_box(model.color_cosine(black_box(0), black_box(1))))
    });
}

fn bench_tiny_nearest(c: &mut Criterion) {
    let model = tiny_c3();
    let q = [41.0_f64, 59.0, -12.0];
    c.bench_function("tiny_color_index_nearest", |b| {
        b.iter(|| black_box(model.color_index(black_box(q))))
    });
}

fn bench_tiny_analyze_palette(c: &mut Criterion) {
    let model = tiny_c3();
    let palette = Array2::from_elem((16, 3), 42.0_f64);
    c.bench_function("tiny_analyze_palette_16_rows", |b| {
        b.iter(|| black_box(model.analyze_palette(palette.clone())))
    });
}

fn bench_tiny_distance_matrix(c: &mut Criterion) {
    let model = tiny_c3();
    let palette = Array2::from_shape_vec((8, 3), vec![40_f64; 8 * 3]).unwrap();
    let samples = model.analyze_palette(palette);
    c.bench_function("tiny_distance_matrix_8_samples", |b| {
        b.iter(|| black_box(model.compute_color_name_distance_matrix(black_box(&samples))))
    });
}

fn bench_optional_full_dataset(c: &mut Criterion) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    if !(dir.join("c3_color.npy").exists()
        && dir.join("c3_a.npy").exists()
        && dir.join("c3_t.npy").exists())
    {
        return;
    }

    let model = C3::from_npy_dir(&dir).expect("read npy");

    c.bench_function("full_color_entropy_7271", |b| {
        b.iter(|| black_box(model.color_entropy(black_box(7271))))
    });

    c.bench_function("full_color_related_terms_top10", |b| {
        b.iter(|| {
            black_box(model.color_related_terms(
                black_box(7271),
                Some(10),
                None,
                None,
            ))
        })
    });

    c.bench_function("full_color_cosine_pair", |b| {
        b.iter(|| black_box(model.color_cosine(black_box(2173), black_box(7271))))
    });

    let q = [60.3_f64, 98.2, -60.8];
    c.bench_function("full_color_index_nearest", |b| {
        b.iter(|| black_box(model.color_index(black_box(q))))
    });
}

criterion_group!(
    benches,
    bench_tiny_entropy,
    bench_tiny_related_terms,
    bench_tiny_cosine,
    bench_tiny_nearest,
    bench_tiny_analyze_palette,
    bench_tiny_distance_matrix,
    bench_optional_full_dataset,
);
criterion_main!(benches);
