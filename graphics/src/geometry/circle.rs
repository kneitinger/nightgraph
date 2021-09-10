use super::Point;

#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Circle {
        Self { center, radius }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn center(&self) -> Point {
        self.center
    }
}
