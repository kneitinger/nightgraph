use super::{Path, PathCommand, Pathable, Point};

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
    fn to_path(&self) -> Path {
        Path::new(self.a, PathCommand::LineTo(self.b))
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
    fn to_path(&self) -> Path {
        let mut path = Path::new(self.points[0], PathCommand::LineTo(self.points[1]));
        for &p in self.points.iter().skip(2) {
            path.line_to(p);
        }
        path
    }
}
