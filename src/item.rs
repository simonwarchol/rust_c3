use kd_tree::KdPoint;

pub(crate) struct Item {
    pub(crate) point: [f64; 3],
    pub(crate) id: usize,
}

impl KdPoint for Item {
    type Scalar = f64;
    type Dim = typenum::U3;

    fn at(&self, k: usize) -> f64 {
        self.point[k]
    }
}
