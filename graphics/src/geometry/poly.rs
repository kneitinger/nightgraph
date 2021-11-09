use super::*;
use kurbo::BezPath;
use kurbo::Shape as KurboShape;

pub struct PolyBuilder {
    points: Option<Vec<Point>>,
    stroke_width: f64,
    smooth: bool,
    precompute: bool,
}

impl PolyBuilder {
    pub fn new() -> Self {
        Self {
            points: None,
            precompute: false,
            smooth: false,
            stroke_width: DEFAULT_STROKE_WIDTH,
        }
    }

    pub fn points(&mut self, points: &[Point]) -> &mut Self {
        self.points = Some(Vec::from(points));
        self
    }

    pub fn precompute(&mut self) -> &mut Self {
        self.precompute = true;
        self
    }

    pub fn smooth(&mut self) -> &mut Self {
        self.smooth = true;
        self
    }

    pub fn stroke_width(&mut self, stroke_width: f64) -> &mut Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn build(&self) -> GeomResult<Poly> {
        if let Some(ps) = &self.points {
            if ps.len() < 2 {
                return Err(GeomError::malformed_path("TODO"));
            }

            let mut poly = if self.smooth {
                Poly::new_smooth(&ps)
            } else {
                Poly::new(&ps)?
            };

            poly.stroke_width = self.stroke_width;

            if self.precompute {
                poly.bounding_box = Some(poly.inner().bounding_box());
            }

            Ok(poly)
        } else {
            Err(GeomError::malformed_path("TODO"))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Poly {
    inner: BezPath,
    bounding_box: Option<kurbo::Rect>,
    stroke_width: f64,
}

impl Poly {
    pub fn new(points: &[Point]) -> GeomResult<Poly> {
        let mut inner = BezPath::new();
        match points {
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
        Ok(Self {
            inner,
            bounding_box: None,
            stroke_width: DEFAULT_STROKE_WIDTH,
        })
    }
    fn inner(&self) -> BezPath {
        self.inner.clone()
    }

    pub fn new_smooth(points: &[Point]) -> Self {
        Poly {
            inner: Path::from_points_smooth_closed(points).as_bezpath(),
            bounding_box: None,
            stroke_width: DEFAULT_STROKE_WIDTH,
        }
    }

    pub fn translate(&self, translation: Vec2) -> Self {
        let ts = kurbo::TranslateScale::new(translation, 1.0);
        Self {
            inner: ts * self.inner.clone(),
            bounding_box: None,
            stroke_width: self.stroke_width,
        }
    }
}

impl Shaped for Poly {
    fn as_shape(&self) -> Shape {
        Shape::Poly(self.clone())
    }
    fn stroke(&self) -> f64 {
        self.stroke_width
    }
    fn to_path(&self) -> Path {
        Path::from(self.inner())
    }
    fn as_bezpath(&self) -> BezPath {
        self.inner.clone()
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
