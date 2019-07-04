pub trait F64Extra {
    fn val(&self) -> f64;
    fn ceil_at(&self, idx: i32) -> f64 {
        let x = self.val();
        let shift = 10f64.powi(idx);
        (x / shift).ceil() * shift
    }
}

impl F64Extra for f64 {
    fn val(&self) -> f64 {
        *self
    }
}

#[test]
fn ceil_at() {
    let params = [
        (1.1, 0, 2.0),
        (1.23, -1, 1.3),
        (1.23, -2, 1.23),
        (123.0, 1, 130.0),
    ];

    for p in &params {
        assert_eq!(p.0.ceil_at(p.1), p.2);
    }
}
