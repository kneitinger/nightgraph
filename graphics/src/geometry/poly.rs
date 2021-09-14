use super::{GeomError, GeomResult, Point, WrapsBez, WrapsShape};
use kurbo::BezPath;
#[derive(Debug)]
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
}

impl WrapsBez for Poly {}
impl WrapsShape<BezPath> for Poly {
    fn inner(&self) -> BezPath {
        self.inner.clone()
    }
    /*
    fn to_path(&self) -> GeomResult<Path> {
        self.sub_polys
            .iter()
            .map(|poly| poly.to_path())
            .collect::<GeomResult<Vec<Path>>>()
            .and_then(|v| {
                v.into_iter()
                    .reduce(|mut a, b| {
                        a.append(&b);
                        a
                    })
                    .ok_or_else(|| GeomError::malformed_poly("complex poly had no sub polys"))
            })
    }
    */
}

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

impl WrapsBez for ComplexPoly {}
impl WrapsShape<BezPath> for ComplexPoly {
    fn inner(&self) -> BezPath {
        self.inner.clone()
    }
}
