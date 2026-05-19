//! Integration tests: tiny in-memory `C3`, optional golden run against `src/c3_*.npy`.

use std::f64::consts::LN_2;
use std::path::Path;

use ndarray::{array, arr2, Array1, Array2};
use rust_c3::{get_c3_terms, ColorSample, C3, RelatedTerm};

fn assert_close(a: f64, b: f64, eps: f64) {
    assert!(
        (a - b).abs() <= eps,
        "expected {b:e}, got {a:e} (diff {})",
        (a - b).abs()
    );
}

fn tiny_c3() -> C3 {
    let w = get_c3_terms().len() as i64;
    let color = Array2::from_shape_vec((3, 3), vec![0, 0, 0, 100, 0, 0, 0, 100, 0]).unwrap();
    let a = Array1::zeros(3);
    let t_vec = array![0i64, 10, 1, 30, w, 5, w + 2, 5];
    C3::from_raw_parts(color, a, t_vec)
}

#[test]
fn in_memory_model() {
    let c3 = tiny_c3();

    let p0 = 0.25_f64;
    let p1 = 0.75_f64;
    let expected_h = p0 * p0.ln() / LN_2 + p1 * p1.ln() / LN_2;
    assert_close(c3.color_entropy(0), expected_h, 1e-12);

    let terms = c3.color_related_terms(0, None, None, None);
    assert_close(terms.iter().map(|t| t.score).sum(), 1.0, 1e-12);
    assert_eq!(
        terms,
        vec![
            RelatedTerm {
                index: 1,
                score: 0.75
            },
            RelatedTerm {
                index: 0,
                score: 0.25
            },
        ]
    );

    let top1 = c3.color_related_terms(0, Some(1), None, None);
    assert_eq!(top1.len(), 1);
    assert_eq!(top1[0].index, 1);
    assert_close(top1[0].score, 0.75, 1e-12);

    assert_close(c3.color_cosine(0, 0), 1.0, 1e-12);
    let cos01 = 50.0_f64 / (1000_f64.sqrt() * 50_f64.sqrt());
    assert_close(c3.color_cosine(0, 1), cos01, 1e-12);
    assert_close(c3.color_cosine(1, 0), cos01, 1e-12);

    assert_eq!(c3.color_index([0.0, 0.0, 0.0]), 0);
    assert_eq!(c3.color_index([100.0, 0.0, 0.0]), 1);
    assert_eq!(c3.color_index([0.0, 100.0, 0.0]), 2);

    let palette = arr2(&[
        [0.0_f64, 0.0, 0.0],
        [100.0, 0.0, 0.0],
        [0.0, 100.0, 0.0],
    ]);
    let samples = c3.analyze_palette(palette);
    assert_eq!(samples.len(), 3);
    let h0 = (c3.color_entropy(0) - (-4.5)) / (0.0 - (-4.5));
    assert_eq!(samples[0], ColorSample { c: 0, h: h0 });

    let d_samples = vec![
        ColorSample { c: 0, h: 0.0 },
        ColorSample { c: 1, h: 0.0 },
        ColorSample { c: 0, h: 0.0 },
    ];
    let m = c3.compute_color_name_distance_matrix(&d_samples);
    assert_close(m[[0, 0]], 0.0, 1e-12);
    assert_close(m[[1, 1]], 0.0, 1e-12);
    assert_close(m[[2, 2]], 0.0, 1e-12);
    assert_close(m[[0, 1]], m[[1, 0]], 1e-12);
    assert_close(m[[0, 2]], 0.0, 1e-12);

    let pt_palette = arr2(&[[0.0_f64, 0.0, 0.0], [100.0, 0.0, 0.0]]);
    let pt_rows = c3.get_palette_terms(pt_palette, 5);
    assert_eq!(pt_rows.len(), 2);
    assert!(pt_rows[0].len() <= 5 && pt_rows[1].len() <= 5);
}

#[test]
#[ignore = "requires src/c3_color.npy, src/c3_a.npy, src/c3_t.npy"]
fn golden_full_dataset() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    assert!(
        dir.join("c3_color.npy").exists(),
        "missing {}; place full C3 npy bundle under src/",
        dir.display()
    );

    let c3 = C3::from_npy_dir(&dir).expect("read npy");

    assert_close(c3.color_entropy(7271), -2.9358562554686025, 1e-9);

    let rt = c3.color_related_terms(7271, Some(10), None, None);
    assert_eq!(rt.len(), 10);
    assert_eq!(rt[0].index, 4);
    assert_close(rt[0].score, 0.31417624521072796, 1e-12);

    assert_close(c3.color_cosine(2173, 7271), 0.009036541917907336, 1e-12);

    assert_eq!(c3.color_index([60.3, 98.2, -60.8]), 7271);

    let palette = Array2::from_shape_vec(
        (3, 3),
        vec![
            60.32273214,
            98.2353325,
            -60.84232404,
            79.42618245,
            -1.22650957,
            -19.14108948,
            66.88027726,
            43.42296322,
            71.85391542,
        ],
    )
    .unwrap();

    let analyzed = c3.analyze_palette(palette.clone());
    assert_eq!(analyzed.len(), 3);

    let matrix = c3.compute_color_name_distance_matrix(&analyzed);
    assert_eq!(matrix.shape(), &[3, 3]);

    let expected_21 = 1.0 - c3.color_cosine(analyzed[2].c, analyzed[1].c);
    assert_close(matrix[[2, 1]], expected_21, 1e-12);
    assert_close(matrix[[2, 1]], matrix[[1, 2]], 1e-12);
}
