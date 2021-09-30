use itertools::Itertools;
use kurbo::{
    flatten, BezPath, Line as KurboLine, ParamCurve, ParamCurveNearest, PathSeg,
    Shape as KurboShape,
};
pub use kurbo::{PathEl, Point, Vec2, DEFAULT_ACCURACY};

mod circle;
mod error;
mod line;
mod path;
mod poly;
mod text;

pub use circle::Circle;
pub use error::*;
pub use line::Line;
pub use path::Path;
pub use poly::Poly;
pub use text::TextBuilder;

/// Convenience function to allow making `Point`s quickly
/// from any compatible number type
pub fn point<T: Into<f64>, U: Into<f64>>(x: T, y: U) -> Point {
    Point::new(x.into(), y.into())
}

pub fn avg_point(p1: Point, p2: Point) -> Point {
    point((p1.x + p2.x) / 2., (p1.y + p2.y) / 2.)
}

pub const DEFAULT_TOLERANCE: f64 = 1e-1;

pub enum Shape {
    Path(Path),
    Circle(Circle),
    Line(Line),
    Poly(Poly),
}

impl<T: Shaped> From<T> for Shape {
    fn from(s: T) -> Shape {
        s.as_shape()
    }
}

impl Shape {
    pub fn new<T: Shaped + Into<Shape>>(s: T) -> Self {
        Shape::from(s)
    }

    fn inner(&self) -> &dyn Shaped {
        match self {
            Self::Path(p) => p,
            Self::Circle(c) => c,
            Self::Line(l) => l,
            Self::Poly(p) => p,
        }
    }
    pub fn area(&self) -> f64 {
        self.inner().area()
    }
    pub fn to_path(&self) -> Path {
        self.inner().to_path()
    }

    pub fn translate(&self, translation: Vec2) -> Self {
        match self {
            Self::Circle(c) => Self::Circle(c.translate(translation)),
            Self::Path(p) => Self::Path(p.translate(translation)),
            Self::Line(l) => Self::Line(l.translate(translation)),
            Self::Poly(p) => Self::Poly(p.translate(translation)),
        }
    }
}

/// Represents the ability to be converted to a path, with optional hatch fill.
pub trait Shaped {
    /// Returns the verticies of the line decomposition of the shape
    fn to_path(&self) -> Path;
    fn as_bezpath(&self) -> BezPath;
    fn perimeter(&self) -> f64;
    fn difference(&self, other: &dyn Shaped) -> Path {
        if self
            .bounding_box()
            .intersect(other.bounding_box())
            .is_empty()
        {
            return self.to_path();
        }

        let self_path = self.as_bezpath();
        let self_segs = self_path.segments();
        let other_flattened = other.to_lines().unwrap();
        let other_lines = other_flattened
            .inner()
            .segments()
            .map(|s| match s {
                PathSeg::Line(l) => l,
                _ => unreachable!(),
            })
            .collect::<Vec<KurboLine>>();

        let mut result_segs = Vec::new();

        for seg in self_segs {
            let mut intersections = Vec::new();
            for line in &other_lines {
                let intersection_info = seg.intersect_line(*line);
                for info in intersection_info {
                    intersections.push(info.segment_t);
                }
            }
            if intersections.is_empty() {
                if !other_flattened.contains(seg.start()) {
                    result_segs.push(seg);
                }
            } else {
                intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mut ts = Vec::new();
                if other_flattened.contains(seg.start()) {
                    ts.push(intersections.remove(0));
                } else {
                    ts.push(0.);
                }
                if other_flattened.contains(seg.end()) {
                    if intersections.len() > 0 {
                        ts.push(intersections.pop().unwrap());
                    }
                    for i in &intersections {
                        ts.insert(ts.len() - 1, *i);
                    }
                } else {
                    for i in &intersections {
                        ts.push(*i);
                    }
                    ts.push(1.0);
                }
                for (t0, t1) in ts.iter().tuples() {
                    result_segs.push(seg.subsegment(*t0..*t1));
                }
            }
        }

        BezPath::from_path_segments(result_segs.into_iter()).into()
    }

    fn intersections(&self, other: &dyn Shaped) -> Vec<Point> {
        if self
            .bounding_box()
            .intersect(other.bounding_box())
            .is_empty()
        {
            return vec![];
        }

        let self_path = self.as_bezpath();
        let self_segs = self_path.segments();
        let other_lines = other
            .to_points()
            .iter()
            .map(|v| {
                v.iter()
                    .tuple_windows()
                    .map(|(a, b)| KurboLine::new(*a, *b))
            })
            .flatten()
            .collect::<Vec<KurboLine>>();

        self_segs
            .map(|seg| {
                other_lines
                    .iter()
                    .map(move |line| {
                        seg.intersect_line(*line)
                            .into_iter()
                            .map(move |i| line.eval(i.line_t))
                    })
                    .flatten()
            })
            .flatten()
            .collect::<Vec<Point>>()
    }

    fn contains(&self, p: Point) -> bool;
    fn area(&self) -> f64;
    fn bounding_box(&self) -> kurbo::Rect;
    fn as_shape(&self) -> Shape;
    fn closest_point(&self, p: Point) -> Point {
        let nearest_info = self
            .as_bezpath()
            .segments()
            .map(|s| {
                let n = s.nearest(p, DEFAULT_ACCURACY);
                (n.distance_sq, n.t, s)
            })
            .reduce(|a, b| if a.0 < b.0 { a } else { b })
            .unwrap();
        nearest_info.2.eval(nearest_info.1)
    }

    // TODO: doesn't adequately report if a path should be closed or not.
    // to fix this, either explicitly re-add the first point to the end
    // of the vec, or include some flag in the return value
    fn to_points(&self) -> Vec<Vec<Point>> {
        let straightened_paths = self.to_path().separate().unwrap();
        let mut point_groups = vec![];
        for p in straightened_paths {
            let mut points = vec![];
            for cmd in p.to_lines().unwrap().commands() {
                match cmd {
                    PathEl::MoveTo(p) | PathEl::LineTo(p) => points.push(*p),
                    _ => {}
                }
            }
            point_groups.push(points);
        }
        point_groups
    }

    fn to_lines(&self) -> GeomResult<Path> {
        let mut path_elements = vec![];
        let callback = |el: PathEl| path_elements.push(el);
        flatten(
            self.to_path().inner().path_elements(DEFAULT_TOLERANCE),
            DEFAULT_TOLERANCE,
            callback,
        );
        Path::from_commands(path_elements.as_slice())
    }

    //fn to_lines(&self) -> GeomResult<Path>;
}
