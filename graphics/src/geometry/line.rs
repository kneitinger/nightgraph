use super::{GeomResult, Path, Point, Shaped, DEFAULT_ACCURACY, DEFAULT_TOLERANCE};
use kurbo::{Line as KurboLine, ParamCurve, Shape as KurboShape};

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
    fn inner(&self) -> KurboLine {
        self.inner
    }
}

impl Shaped for Line {
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
