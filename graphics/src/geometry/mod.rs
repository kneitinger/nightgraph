use kurbo::Shape;
pub use kurbo::{flatten, BezPath, PathEl, Point, DEFAULT_ACCURACY};
use std::error::Error;
use std::fmt;

mod circle;
pub use circle::Circle;
mod line;
pub use line::{Line, MultiLine};
mod poly;
pub use poly::{ComplexPoly, Poly};
mod text;
pub use text::TextBuilder;

/// Convenience function to allow making `Point`s quickly
/// from any compatible number type
pub fn point<T: Into<f64>, U: Into<f64>>(x: T, y: U) -> Point {
    Point::new(x.into(), y.into())
}

pub type GeomResult<T> = Result<T, GeomError>;

pub const DEFAULT_TOLERANCE: f64 = 0.05;
#[derive(Debug)]
pub enum GeomError {
    PathError(String),
    MalformedPath(String),
    MalformedPoly(String),
    FontError(String),
    IoError(std::io::Error),
}
impl From<std::io::Error> for GeomError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl GeomError {
    fn path_error(msg: &str) -> Self {
        Self::PathError(msg.to_string())
    }
    #[allow(dead_code)]
    fn malformed_poly(msg: &str) -> Self {
        Self::MalformedPoly(msg.to_string())
    }
    fn malformed_path(msg: &str) -> Self {
        Self::MalformedPath(msg.to_string())
    }
    fn font_error(msg: &str) -> Self {
        Self::FontError(msg.to_string())
    }
}

impl fmt::Display for GeomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PathError(msg) => write!(f, "PathError: {}", msg),
            Self::MalformedPoly(msg) => write!(f, "MalformedPoly: {}", msg),
            Self::MalformedPath(msg) => write!(f, "MalformedPath: {}", msg),
            Self::FontError(msg) => write!(f, "FontError: {}", msg),
            Self::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl Error for GeomError {}

#[derive(Clone, Debug)]
pub struct Path {
    inner: BezPath,
}

impl WrapsBez for Path {}
impl WrapsShape<BezPath> for Path {
    fn inner(&self) -> BezPath {
        self.inner.clone()
    }
}

impl From<BezPath> for Path {
    fn from(bez_path: BezPath) -> Self {
        Self { inner: bez_path }
    }
}

impl Path {
    pub fn new(origin: Point, cmd: PathEl) -> Self {
        let mut inner = BezPath::new();
        inner.move_to(origin);
        inner.push(cmd);
        Self { inner }
    }

    // TODO: there needs to be an error if the path doesn't start with a move_to
    pub fn with_commands(commands: &[PathEl]) -> GeomResult<Self> {
        match commands {
            [PathEl::MoveTo(_), _, ..] => Ok(Self {
                inner: BezPath::from_vec(Vec::from(commands)),
            }),
            [_, ..] => Err(GeomError::path_error(
                "paths must start with a MoveTo command",
            )),
            _ => Err(GeomError::path_error("path requires at least 2 commands")),
        }
    }

    pub fn commands(&self) -> &[PathEl] {
        self.inner.elements()
    }

    pub fn move_to(&mut self, point: Point) {
        self.inner.push(PathEl::MoveTo(point));
    }
    pub fn line_to(&mut self, endpoint: Point) {
        self.inner.push(PathEl::LineTo(endpoint));
    }
    pub fn quad_to(&mut self, ctrl_point: Point, endpoint: Point) {
        self.inner.push(PathEl::QuadTo(ctrl_point, endpoint));
    }
    pub fn curve_to(&mut self, ctrl_point_0: Point, ctrl_point_1: Point, endpoint: Point) {
        self.inner
            .push(PathEl::CurveTo(ctrl_point_0, ctrl_point_1, endpoint));
    }

    pub fn curve_through(&mut self, _endpoint: Point) {
        // https://www.particleincell.com/2012/bezier-splines/
        unimplemented!();
    }

    pub fn close(&mut self) {
        self.inner.push(PathEl::ClosePath);
    }

    pub fn append(&mut self, other: &Self) {
        for cmd in &other.inner {
            self.inner.push(cmd);
        }
    }

    pub fn closed(&self) -> bool {
        matches!(self.inner.elements().last(), Some(PathEl::ClosePath))
    }

    pub fn separate(&self) -> GeomResult<Vec<Path>> {
        let cmds = self.inner.elements();
        let mut paths = vec![];

        let mut path_cmds = vec![];

        for &cmd in cmds {
            match cmd {
                PathEl::MoveTo(_) => {
                    if path_cmds.is_empty() {
                        path_cmds.push(cmd)
                    } else if matches!(path_cmds.as_slice(), [PathEl::MoveTo(_)]) {
                        path_cmds.clear();
                        path_cmds.push(cmd);
                    } else {
                        paths.push(Path::with_commands(path_cmds.as_slice())?);
                        path_cmds.clear();
                        path_cmds.push(cmd);
                    }
                }
                _ => path_cmds.push(cmd),
            }
        }
        paths.push(Path::with_commands(path_cmds.as_slice())?);

        Ok(paths)
    }
}

pub trait WrapsBez: WrapsShape<BezPath> {}

/// Represents the ability to be converted to a path, with optional hatch fill.
pub trait WrapsShape<T: Shape> {
    fn inner(&self) -> T;
    /// Returns the verticies of the line decomposition of the shape
    fn to_path(&self) -> Path {
        Path {
            inner: self.inner().into_path(DEFAULT_TOLERANCE),
        }
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
            self.inner().path_elements(DEFAULT_TOLERANCE),
            DEFAULT_TOLERANCE,
            callback,
        );
        Path::with_commands(path_elements.as_slice())
    }

    fn perimeter(&self) -> f64 {
        self.inner().perimeter(DEFAULT_ACCURACY)
    }

    fn contains(&self, p: Point) -> bool {
        self.inner().contains(p)
    }

    fn area(&self) -> f64 {
        self.inner().area()
    }
}
