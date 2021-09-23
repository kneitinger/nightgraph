pub use kurbo::Shape as KurboShape;
use kurbo::{flatten, BezPath};

pub use kurbo::{PathEl, Point, DEFAULT_ACCURACY};

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

pub struct Shape {
    wrapped: Box<dyn Shaped>,
}

impl<T: 'static + Shaped> From<T> for Shape {
    fn from(s: T) -> Shape {
        Shape::new(s)
    }
}

impl Shape {
    pub fn new<T: 'static + Shaped>(s: T) -> Self {
        Shape {
            wrapped: Box::new(s),
        }
    }
    pub fn area(&self) -> f64 {
        self.wrapped.area()
    }
    pub fn to_path(&self) -> Path {
        self.wrapped.to_path()
    }
}

/// Represents the ability to be converted to a path, with optional hatch fill.
pub trait Shaped {
    /// Returns the verticies of the line decomposition of the shape
    fn to_path(&self) -> Path;
    fn perimeter(&self) -> f64;
    fn contains(&self, p: Point) -> bool;
    fn area(&self) -> f64;

    // TODO: doesn't adequately report if a path should be closed or not.
    // to fix this, either explicitly re-add the first point to the end
    // of the vec, or include some flag in the return value
    /*
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
            self.inner_bez().path_elements(DEFAULT_TOLERANCE),
            DEFAULT_TOLERANCE,
            callback,
        );
        Path::with_commands(path_elements.as_slice())
    }
    */

    //fn to_lines(&self) -> GeomResult<Path>;
}
