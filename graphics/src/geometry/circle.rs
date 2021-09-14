use super::{Point, WrapsShape};
use kurbo::Circle as KurboCircle;

#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: f64,
    inner: KurboCircle,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Circle {
        Self {
            center,
            radius,
            inner: KurboCircle::new(center, radius),
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn center(&self) -> Point {
        self.center
    }
}

impl WrapsShape<KurboCircle> for Circle {
    fn inner(&self) -> KurboCircle {
        self.inner
    }
}
