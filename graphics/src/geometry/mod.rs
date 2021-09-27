use kurbo::flatten;
pub use kurbo::Shape as KurboShape;

pub use kurbo::{
    BezPath, Line as KurboLine, ParamCurve, ParamCurveNearest, PathEl, Point, Vec2,
    DEFAULT_ACCURACY,
};

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

pub const DEFAULT_TOLERANCE: f64 = 0.05;

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
    /*
    pub fn translate(&self, v: crate::units::Vec2) -> Self {
        let trans = kurbo::TranslateScale::translate(v);
        Self::from(match self {
            Self::Path(p) => Self::from(p.inner() * trans),
            Self::Circle(c) => Self::from(c * trans),
            Self::Line(l) => Self::from(l * trans),
            Self::Poly(p) => Self::from(p * trans),
        })
    }
    */

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
    fn contains(&self, p: Point) -> bool;
    fn area(&self) -> f64;
    fn bounding_box(&self) -> kurbo::Rect;
    fn as_shape(&self) -> Shape;

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
        Path::with_commands(path_elements.as_slice())
    }

    //fn to_lines(&self) -> GeomResult<Path>;
}
