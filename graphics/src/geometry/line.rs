use super::{GeomResult, Path, Point, Shape, Shaped, Vec2, DEFAULT_ACCURACY, DEFAULT_TOLERANCE};
use kurbo::{BezPath, Line as KurboLine, ParamCurve, Shape as KurboShape};

#[derive(Copy, Clone, Debug)]
pub struct Line {
    inner: KurboLine,
}

impl Line {
    pub fn new(a: Point, b: Point) -> GeomResult<Line> {
        Ok(Self {
            inner: KurboLine::new(a, b),
        })
    }

    pub fn lerp(&self, t: f64) -> Point {
        self.inner.eval(t)
    }
    pub fn inner(&self) -> KurboLine {
        self.inner
    }

    pub fn p0(&self) -> Point {
        self.inner.p0
    }
    pub fn p1(&self) -> Point {
        self.inner.p1
    }

    pub fn translate(&self, translation: Vec2) -> Self {
        let ts = kurbo::TranslateScale::new(translation, 1.0);
        Self {
            inner: ts * self.inner,
        }
    }
}

impl Shaped for Line {
    fn as_shape(&self) -> Shape {
        Shape::Line(*self)
    }
    fn to_path(&self) -> Path {
        Path::from(self.inner.into_path(DEFAULT_TOLERANCE))
    }
    fn as_bezpath(&self) -> BezPath {
        self.inner.into_path(DEFAULT_TOLERANCE)
    }
    fn bounding_box(&self) -> kurbo::Rect {
        self.inner.bounding_box()
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
