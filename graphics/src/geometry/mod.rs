use euclid::Point2D;
use itertools::Itertools;
use num_traits::ToPrimitive;
use std::error::Error;
use std::fmt;

mod circle;
pub use circle::Circle;
mod line;
pub use line::{Line, MultiLine};
mod poly;
pub use poly::{ComplexPoly, Poly};
mod text;
pub use text::Text;

pub struct PageSpace;
pub type Point = Point2D<f64, PageSpace>;

/// Convenience function to allow making `Point`s quickly
/// from any compatible number type
pub fn point<T: ToPrimitive, U: ToPrimitive>(x: T, y: U) -> Point {
    Point::new(x.to_f64().unwrap(), y.to_f64().unwrap())
}

pub type GeomResult<T> = Result<T, GeomError>;

#[derive(Copy, Clone, Debug)]
pub enum PathCommand {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point),
    CurveTo(Point, Point, Point),
    Close,
}

#[derive(Debug, Clone)]
pub enum GeomError {
    PathError(String),
    MalformedPath(String),
    MalformedPoly(String),
}

impl GeomError {
    fn path_error(msg: &str) -> Self {
        Self::PathError(msg.to_string())
    }
    fn malformed_poly(msg: &str) -> Self {
        Self::MalformedPoly(msg.to_string())
    }
    fn malformed_path(msg: &str) -> Self {
        Self::MalformedPath(msg.to_string())
    }
}

impl fmt::Display for GeomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PathError(msg) => write!(f, "PathError: {}", msg),
            Self::MalformedPoly(msg) => write!(f, "MalformedPoly: {}", msg),
            Self::MalformedPath(msg) => write!(f, "MalformedPath: {}", msg),
        }
    }
}

impl Error for GeomError {
    fn description(&self) -> &str {
        match self {
            Self::PathError(msg) => msg,
            Self::MalformedPoly(msg) => msg,
            Self::MalformedPath(msg) => msg,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    commands: Vec<PathCommand>,
}

impl Path {
    pub fn new(origin: Point, cmd: PathCommand) -> Self {
        Self {
            commands: vec![PathCommand::MoveTo(origin), cmd],
        }
    }

    // TODO: there needs to be an error if the path doesn't start with a move_to
    pub fn with_commands(commands: &[PathCommand]) -> GeomResult<Self> {
        match commands {
            [PathCommand::MoveTo(_), _, ..] => Ok(Self {
                commands: commands.to_owned(),
            }),
            [_, ..] => Err(GeomError::path_error(
                "paths must start with a MoveTo command",
            )),
            _ => Err(GeomError::path_error("path requires at least 2 commands")),
        }
    }

    pub fn commands(&self) -> &[PathCommand] {
        &self.commands
    }

    pub fn move_to(&mut self, point: Point) {
        self.commands.push(PathCommand::MoveTo(point));
    }
    pub fn line_to(&mut self, endpoint: Point) {
        self.commands.push(PathCommand::LineTo(endpoint));
    }
    pub fn quad_to(&mut self, ctrl_point: Point, endpoint: Point) {
        self.commands
            .push(PathCommand::QuadTo(ctrl_point, endpoint));
    }
    pub fn curve_to(&mut self, ctrl_point_0: Point, ctrl_point_1: Point, endpoint: Point) {
        self.commands
            .push(PathCommand::CurveTo(ctrl_point_0, ctrl_point_1, endpoint));
    }

    pub fn curve_through(&mut self, _endpoint: Point) {
        // https://www.particleincell.com/2012/bezier-splines/
        unimplemented!();
    }

    pub fn close(&mut self) {
        self.commands.push(PathCommand::Close);
    }

    pub fn append(&mut self, other: &Self) {
        for cmd in &other.commands {
            self.commands.push(*cmd);
        }
    }

    pub fn closed(&self) -> bool {
        matches!(self.commands.last(), Some(PathCommand::Close))
    }

    pub fn separate(&self) -> GeomResult<Vec<Path>> {
        let cmds = &self.commands;
        let mut paths = vec![];

        let mut path_cmds = vec![];

        for &cmd in cmds {
            match cmd {
                PathCommand::MoveTo(_) => {
                    if path_cmds.is_empty() {
                        path_cmds.push(cmd)
                    } else if matches!(path_cmds.as_slice(), [PathCommand::MoveTo(_)]) {
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

impl Pathable for Path {
    fn to_path(&self) -> GeomResult<Path> {
        Ok(self.clone())
    }
}

fn decasteljau(lines: Vec<Line>, t: f64) -> Point {
    let new_lines: Vec<Line> = lines
        .iter()
        .tuple_windows()
        .map(|(la, lb)| Line::new(la.lerp(t), lb.lerp(t)))
        .collect();

    if new_lines.len() == 1 {
        new_lines[0].lerp(t)
    } else {
        decasteljau(new_lines, t)
    }
}

pub trait Pointable {
    fn to_points(&self) -> GeomResult<Vec<Point>>;
}

impl<T: Pathable> Pointable for T {
    fn to_points(&self) -> GeomResult<Vec<Point>> {
        let mut points = vec![];
        let mut last_move_to = None;
        for &cmd in self.to_path()?.commands() {
            match cmd {
                PathCommand::MoveTo(p) => {
                    points.push(p);
                    last_move_to = Some(p);
                }
                PathCommand::LineTo(p) => points.push(p),
                PathCommand::QuadTo(c, p) => {
                    let prev_point = points.last().expect("");
                    let init_points = vec![Line::new(*prev_point, c), Line::new(c, p)];
                    points.append(
                        &mut (1..=100)
                            .map(|n| decasteljau(init_points.clone(), n as f64 / 100.))
                            .collect(),
                    );
                }
                PathCommand::CurveTo(c1, c2, p) => {
                    let prev_point = points.last().expect("");
                    let init_points = vec![
                        Line::new(*prev_point, c1),
                        Line::new(c1, c2),
                        Line::new(c2, p),
                    ];
                    points.append(
                        &mut (1..=100)
                            .map(|n| decasteljau(init_points.clone(), n as f64 / 100.))
                            .collect(),
                    );
                }
                PathCommand::Close => {
                    if let Some(p) = last_move_to {
                        points.push(p)
                    }
                }
            }
        }
        Ok(points)
    }
}

/// Represents the ability to be converted to a path, with optional hatch fill.
pub trait Pathable {
    /// Returns the verticies of the line decomposition of the shape
    fn to_path(&self) -> GeomResult<Path>;
}
