use euclid::Point2D;
use itertools::Itertools;
use num_traits::ToPrimitive;

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

#[derive(Copy, Clone, Debug)]
pub enum PathCommand {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point),
    CurveTo(Point, Point, Point),
    Close,
}

#[derive(Clone, Debug)]
pub struct Path {
    commands: Vec<PathCommand>,
}

impl Path {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    // TODO: there needs to be an error if the path doesn't start with a move_to
    pub fn with_commands(commands: Vec<PathCommand>) -> Self {
        Self { commands }
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

    pub fn append(&mut self, other: Self) {
        for cmd in other.commands {
            self.commands.push(cmd);
        }
    }
}

impl Pathable for Path {
    fn to_path(&self) -> Path {
        self.clone()
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
    fn to_points(&self) -> Vec<Point>;
}

impl<T: Pathable> Pointable for T {
    fn to_points(&self) -> Vec<Point> {
        let mut points = vec![];
        for &cmd in self.to_path().commands() {
            match cmd {
                PathCommand::MoveTo(p) => points.push(p),
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
                PathCommand::Close => {}
            }
        }
        points
    }
}

/// Represents the ability to be converted to a path, with optional hatch fill.
pub trait Pathable {
    /// Returns the verticies of the line decomposition of the shape
    fn to_path(&self) -> Path;
}
