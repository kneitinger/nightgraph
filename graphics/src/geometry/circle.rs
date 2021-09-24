use super::DEFAULT_ACCURACY;
use super::{Path, Point, Shape, Shaped, DEFAULT_TOLERANCE};
use kurbo::Circle as KurboCircle;
use kurbo::Shape as KurboShape;

#[derive(Debug, Clone)]
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
    pub fn inner(&self) -> KurboCircle {
        self.inner
    }
}
impl Shaped for Circle {
    fn as_shape(&self) -> Shape {
        Shape::Circle(self.clone())
    }
    fn to_path(&self) -> Path {
        Path::from(self.inner.into_path(DEFAULT_TOLERANCE))
    }
    fn perimeter(&self) -> f64 {
        self.inner.perimeter(DEFAULT_ACCURACY)
    }
    fn contains(&self, p: Point) -> bool {
        self.inner.contains(p)
    }
    fn area(&self) -> f64 {
        self.inner.area()
    }
}
