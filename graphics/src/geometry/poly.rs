use super::{GeomError, GeomResult, Path, Point, Shape, Shaped, DEFAULT_ACCURACY};
use kurbo::BezPath;
use kurbo::Shape as KurboShape;

#[derive(Debug, Clone)]
pub struct Poly {
    inner: BezPath,
}

impl Poly {
    pub fn new(points: Vec<Point>) -> GeomResult<Poly> {
        let mut inner = BezPath::new();
        match points.as_slice() {
            [first, second, third, rest @ ..] => {
                inner.move_to(*first);
                inner.line_to(*second);
                inner.line_to(*third);
                for p in rest {
                    inner.line_to(*p);
                }
                inner.close_path();
            }
            _ => return Err(GeomError::malformed_path("Poly has less than three points")),
        }
        Ok(Self { inner })
    }
    fn inner(&self) -> BezPath {
        self.inner.clone()
    }

    pub fn translate(&self, translation: Vec2) -> Self {
        let ts = kurbo::TranslateScale::new(translation, 1.0);
        Self {
            inner: ts * self.inner.clone(),
        }
    }
}

impl Shaped for Poly {
    fn as_shape(&self) -> Shape {
        Shape::Poly(self.clone())
    }
    fn to_path(&self) -> Path {
        Path::from(self.inner())
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
    fn bounding_box(&self) -> kurbo::Rect {
        self.inner.bounding_box()
    }
}

/*

// TODO: see if this is actually needed.  Can poly alone cover all the
// cases (holes etc.)
#[derive(Debug)]
pub struct ComplexPoly {
    sub_polys: Vec<Poly>,
    inner: BezPath,
}

impl ComplexPoly {
    pub fn new(sub_polys: Vec<Poly>) -> ComplexPoly {
        Self {
            sub_polys,
            inner: BezPath::new(),
        }
    }
}

impl WrapsShape for ComplexPoly {
    type Inner = BezPath;
    fn inner(&self) -> Self::Inner {
        self.inner.clone()
    }
}

*/
