use super::Vec2;
use super::{Path, Point, Shape, Shaped, DEFAULT_TOLERANCE};
use super::{DEFAULT_ACCURACY, DEFAULT_STROKE_WIDTH};
use kurbo::BezPath;
use kurbo::Ellipse as KurboEllipse;
use kurbo::Shape as KurboShape;

#[derive(Debug, Clone, Copy)]
pub struct Ellipse {
    inner: KurboEllipse,
    stroke_width: f64,
}

impl Ellipse {
    pub fn new(center: impl Into<Point>, radii: impl Into<Vec2>, rot: f64) -> Ellipse {
        Self {
            inner: KurboEllipse::new(center, radii, rot),
            stroke_width: DEFAULT_STROKE_WIDTH,
        }
    }

    pub fn radii(&self) -> Vec2 {
        self.inner.radii()
    }
    pub fn center(&self) -> Point {
        self.inner.center()
    }
    pub fn inner(&self) -> KurboEllipse {
        self.inner
    }

    pub fn translate(&self, translation: Vec2) -> Self {
        Self {
            inner: KurboEllipse::new(
                self.inner.center() + translation,
                self.inner.radii(),
                self.inner.rotation(),
            ),
            stroke_width: self.stroke_width,
        }
    }
}

impl Shaped for Ellipse {
    fn bounding_box(&self) -> kurbo::Rect {
        self.inner.bounding_box()
    }
    fn as_shape(&self) -> Shape {
        Shape::Ellipse(*self)
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
    fn stroke(&self) -> f64 {
        self.stroke_width
    }
}
