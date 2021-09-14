use super::{GeomError, GeomResult, Point, WrapsBez, WrapsShape};
use kurbo::Line as KurboLine;
use kurbo::{BezPath, ParamCurve};

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
}

impl WrapsShape<KurboLine> for Line {
    fn inner(&self) -> KurboLine {
        self.inner
    }
}

#[derive(Debug)]
pub struct MultiLine {
    inner: BezPath,
}

impl MultiLine {
    pub fn new(points: Vec<Point>) -> GeomResult<MultiLine> {
        let mut inner = BezPath::new();
        match points.as_slice() {
            [first, second] => {
                inner.move_to(*first);
                inner.line_to(*second)
            }
            [first, second, rest @ ..] => {
                inner.move_to(*first);
                inner.line_to(*second);
                for p in rest {
                    inner.line_to(*p);
                }
            }
            _ => {
                return Err(GeomError::malformed_path(
                    "MultiLine has less than two points",
                ))
            }
        }
        Ok(Self { inner })
    }

    pub fn push_point(&mut self, point: Point) {
        self.inner.line_to(point);
    }
}

impl WrapsBez for MultiLine {}
impl WrapsShape<BezPath> for MultiLine {
    fn inner(&self) -> BezPath {
        self.inner.clone()
    }
}
