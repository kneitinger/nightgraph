use super::{GeomError, GeomResult, Path, PathCommand, Pathable, Point};

#[derive(Copy, Clone, Debug)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Line {
        Self { a, b }
    }

    pub fn lerp(&self, t: f64) -> Point {
        self.a.lerp(self.b, t)
    }
}

impl Pathable for Line {
    fn to_path(&self) -> GeomResult<Path> {
        Ok(Path::new(self.a, PathCommand::LineTo(self.b)))
    }
}

#[derive(Debug)]
pub struct MultiLine {
    points: Vec<Point>,
}

impl MultiLine {
    pub fn new(points: Vec<Point>) -> MultiLine {
        Self { points }
    }

    pub fn push_point(&mut self, point: Point) {
        self.points.push(point);
    }
}

impl Pathable for MultiLine {
    fn to_path(&self) -> GeomResult<Path> {
        match self.points.as_slice() {
            [first, second] => Ok(Path::new(*first, PathCommand::LineTo(*second))),
            [first, second, rest @ ..] => {
                let mut path = Path::new(*first, PathCommand::LineTo(*second));
                for &p in rest {
                    path.line_to(p);
                }
                Ok(path)
            }
            _ => Err(GeomError::malformed_path(
                "MultiLine has less than two points",
            )),
        }
    }
}
