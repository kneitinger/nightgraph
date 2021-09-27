use super::Vec2;
use super::DEFAULT_ACCURACY;
use super::{Path, Point, Shape, Shaped, DEFAULT_TOLERANCE};
use kurbo::BezPath;
use kurbo::Circle as KurboCircle;
use kurbo::Shape as KurboShape;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    inner: KurboCircle,
}

impl Circle {
    pub fn new(center: impl Into<Point>, radius: f64) -> Circle {
        Self {
            inner: KurboCircle::new(center, radius),
        }
    }

    pub fn radius(&self) -> f64 {
        self.inner.radius
    }
    pub fn center(&self) -> Point {
        self.inner.center
    }
    pub fn inner(&self) -> KurboCircle {
        self.inner
    }

    pub fn translate(&self, translation: Vec2) -> Self {
        let ts = kurbo::TranslateScale::new(translation, 1.0);
        Self {
            inner: ts * self.inner,
        }
    }
}
impl Shaped for Circle {
    fn bounding_box(&self) -> kurbo::Rect {
        self.inner.bounding_box()
    }
    fn as_shape(&self) -> Shape {
        Shape::Circle(*self)
    }
    fn to_path(&self) -> Path {
        Path::from(self.inner.into_path(DEFAULT_TOLERANCE))
    }
    fn as_bezpath(&self) -> BezPath {
        self.inner.into_path(DEFAULT_TOLERANCE)
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
