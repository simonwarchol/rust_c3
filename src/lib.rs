//! C3 color naming / palette analysis (Rust port).
//!
//! Data files default to `src/c3_{color,a,t}.npy` under the crate manifest (see [`C3::new`]).
#![forbid(unsafe_code)]

mod item;

pub mod c3_terms;

pub use c3_terms::{get_c3_terms, C3_TERM_STRS};

use std::collections::HashMap;
use std::f64::consts::LN_2;
use std::path::Path;

use item::Item;
use kd_tree::KdTree;
use ndarray::{Array1, Array2};
use ndarray_npy::read_npy;

/// Labeled color slot index and normalized entropy **h** for a palette sample.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSample {
    pub c: usize,
    pub h: f64,
}

/// Term index within **W** and salience **score** (sums to 1 over returned terms for that color).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RelatedTerm {
    pub index: usize,
    pub score: f64,
}

#[allow(dead_code)]
pub struct C3 {
    color: Array2<i64>,
    c: usize,
    w: usize,
    a: Array1<f64>,
    /// Row-major counts: `tw[color * w + term]`.
    tw: Vec<i64>,
    min_e: f64,
    max_e: f64,
    color_count: Array1<i64>,
    terms_count: Array1<i64>,
    tree: KdTree<Item>,
}

#[allow(clippy::new_without_default)]
impl C3 {
    /// Load bundled `.npy` files from `src/` relative to this crate (same paths as before).
    pub fn new() -> C3 {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        Self::from_npy_dir(&dir).unwrap_or_else(|e| {
            panic!(
                "failed to load C3 npy files from {}: {e}",
                dir.display()
            )
        })
    }

    /// Load `c3_color.npy`, `c3_a.npy`, and `c3_t.npy` from `directory`.
    pub fn from_npy_dir(directory: impl AsRef<Path>) -> Result<C3, ndarray_npy::ReadNpyError> {
        let d = directory.as_ref();
        let color: Array2<i64> = read_npy(d.join("c3_color.npy"))?;
        let a: Array1<f64> = read_npy(d.join("c3_a.npy"))?;
        let t_vec: Array1<i64> = read_npy(d.join("c3_t.npy"))?;
        Ok(Self::from_raw_parts(color, a, t_vec))
    }

    /// Construct from in-memory arrays (tests, tooling). `t_vec` is `[k0, v0, k1, v1, …]` like `c3_t.npy`.
    pub fn from_raw_parts(color: Array2<i64>, a: Array1<f64>, t_vec: Array1<i64>) -> C3 {
        let _c = color.shape()[0];
        let _w = c3_terms::get_c3_terms().len();
        assert_eq!(
            color.shape()[1],
            3,
            "color array must have shape (C, 3)"
        );
        assert!(
            t_vec.len().is_multiple_of(2),
            "t_vec must have even length (key, value pairs)"
        );

        let mut tw = vec![0i64; _c * _w];
        let mut color_count: Array1<i64> = Array1::zeros(_c);
        let mut terms_count: Array1<i64> = Array1::zeros(_w);

        for pair in t_vec.exact_chunks(2) {
            let key = pair[0];
            let val = pair[1];
            let ci = (key / _w as i64) as usize;
            let wi = (key % _w as i64) as usize;
            if ci < _c && wi < _w {
                tw[ci * _w + wi] = val;
            }
            color_count[(key as f64 / _w as f64).floor() as usize] += val;
            terms_count[(key % _w as i64) as usize] += val;
        }

        let pts = color
            .outer_iter()
            .enumerate()
            .map(|(i, row)| Item {
                point: [row[0] as f64, row[1] as f64, row[2] as f64],
                id: i,
            })
            .collect();
        let tree: KdTree<Item> = KdTree::build_by_ordered_float(pts);

        C3 {
            c: _c,
            color,
            a,
            tw,
            w: _w,
            min_e: -4.5,
            max_e: 0.0,
            color_count,
            terms_count,
            tree,
        }
    }

    pub fn color_entropy(&self, c: usize) -> f64 {
        let row = c * self.w;
        let denom = self.color_count[c] as f64;
        let mut h = 0.0;
        for w in 0..self.w {
            let count = self.tw[row + w];
            if count > 0 {
                let p = count as f64 / denom;
                h += p * f64::ln(p) / LN_2;
            }
        }
        h
    }

    pub fn color_related_terms(
        &self,
        c: usize,
        limit: Option<usize>,
        min_count: Option<usize>,
        salience_threshold: Option<f64>,
    ) -> Vec<RelatedTerm> {
        let base = c * self.w;
        let mut pairs: Vec<(usize, i64)> = Vec::with_capacity(self.w);
        let mut sum = 0i64;
        for w in 0..self.w {
            let cnt = self.tw[base + w];
            if cnt > 0 {
                pairs.push((w, cnt));
                sum += cnt;
            }
        }
        let sumf = sum as f64;
        let mut filtered_list: Vec<RelatedTerm> = pairs
            .into_iter()
            .map(|(index, cnt)| RelatedTerm {
                index,
                score: cnt as f64 / sumf,
            })
            .collect();

        if let Some(threshold) = salience_threshold {
            filtered_list.retain(|x| x.score > threshold);
        }
        if let Some(min_count) = min_count {
            filtered_list.retain(|x| self.terms_count[x.index] > min_count as i64);
        }
        filtered_list.sort_by(|a, b| b.score.total_cmp(&a.score));
        if let Some(limit) = limit {
            filtered_list.truncate(limit);
        }
        filtered_list
    }

    pub fn color_cosine(&self, a: usize, b: usize) -> f64 {
        let ra = a * self.w;
        let rb = b * self.w;
        let mut sa = 0.0;
        let mut sb = 0.0;
        let mut sc = 0.0;
        for w in 0..self.w {
            let ta = self.tw[ra + w] as f64;
            let tb = self.tw[rb + w] as f64;
            sa += ta * ta;
            sb += tb * tb;
            sc += ta * tb;
        }
        let denom = sa.sqrt() * sb.sqrt();
        if denom == 0.0 {
            f64::NAN
        } else {
            sc / denom
        }
    }

    pub fn color_index(&self, c: [f64; 3]) -> usize {
        self.tree.nearest(&c).unwrap().item.id
    }

    pub fn color(&self, lab: [f64; 3]) -> ColorSample {
        let c = self.color_index(lab);
        let h = (self.color_entropy(c) - self.min_e) / (self.max_e - self.min_e);
        ColorSample { c, h }
    }

    pub fn analyze_palette(&self, palette: Array2<f64>) -> Vec<ColorSample> {
        palette
            .outer_iter()
            .map(|row| self.color([row[0], row[1], row[2]]))
            .collect()
    }

    pub fn get_palette_terms(
        &self,
        palette: Array2<f64>,
        color_term_limit: usize,
    ) -> Vec<Vec<RelatedTerm>> {
        palette
            .outer_iter()
            .map(|row| {
                let c = self.color_index([row[0], row[1], row[2]]);
                self.color_related_terms(c, Some(color_term_limit), None, None)
            })
            .collect()
    }

    pub fn compute_color_name_distance_matrix(&self, data: &[ColorSample]) -> Array2<f64> {
        let n = data.len();
        let mut matrix = Array2::zeros((n, n));
        let mut cache: HashMap<(usize, usize), f64> = HashMap::new();

        for i in 0..n {
            let ci = data[i].c;
            for j in 0..i {
                let cj = data[j].c;
                let cos = if ci == cj {
                    1.0
                } else {
                    let key = if ci < cj { (ci, cj) } else { (cj, ci) };
                    *cache
                        .entry(key)
                        .or_insert_with(|| self.color_cosine(ci, cj))
                };
                let d = 1.0 - cos;
                matrix[[i, j]] = d;
                matrix[[j, i]] = d;
            }
        }
        matrix
    }
}
