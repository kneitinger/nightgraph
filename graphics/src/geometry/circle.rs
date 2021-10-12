use super::Vec2;
use super::DEFAULT_ACCURACY;
use super::{Path, Point, Shape, Shaped, DEFAULT_TOLERANCE};
use kurbo::BezPath;
use kurbo::Circle as KurboCircle;
use kurbo::Shape as KurboShape;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    inner: KurboCircle,
}

impl Circle {
    pub fn new(center: impl Into<Point>, radius: f64) -> Circle {
        Self {
            inner: KurboCircle::new(center, radius),
        }
    }

    pub fn radius(&self) -> f64 {
        self.inner.radius
    }
    pub fn center(&self) -> Point {
        self.inner.center
    }
    pub fn inner(&self) -> KurboCircle {
        self.inner
    }

    pub fn translate(&self, translation: Vec2) -> Self {
        let ts = kurbo::TranslateScale::new(translation, 1.0);
        Self {
            inner: ts * self.inner,
        }
    }
}

impl Shaped for Circle {
    fn bounding_box(&self) -> kurbo::Rect {
        self.inner.bounding_box()
    }
    fn as_shape(&self) -> Shape {
        Shape::Circle(*self)
    }
    fn to_path(&self) -> Path {
        Path::from(self.inner.into_path(DEFAULT_TOLERANCE))
    }
    fn as_bezpath(&self) -> BezPath {
        self.inner.into_path(DEFAULT_TOLERANCE)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::point;
    #[test]
    fn circle_perimeter() {
        let c = Circle::new((10., 10.), 10.);
        assert!((2. * 10. * std::f64::consts::PI - c.perimeter()).abs() < std::f64::EPSILON);
    }

    #[test]
    fn circle_contains() {
        let c = Circle::new((10., 10.), 10.);
        assert!(c.contains((5., 5.).into()));
        assert!(!c.contains((0., 0.).into()));
    }

    fn point_rel_eq(p0: Point, p1: Point, margin: f64) -> bool {
        let sub = p0 - p1;
        sub.x.abs() < margin && sub.y.abs() < margin
    }

    #[test]
    fn circle_closest_point() {
        let c = Circle::new((10., 10.), 5.);
        let accuracy_margin = 1e-3;
        for (test_point, expected_point) in [
            ((10., 0.), (10., 5.)),
            ((0., 10.), (5., 10.)),
            ((11., 10.), (15., 10.)),
            ((9., 10.), (5., 10.)),
        ]
        .map(|(a, b)| (a.into(), b.into()))
        {
            let result_point = c.closest_point(test_point);
            assert!(
                point_rel_eq(expected_point, result_point, accuracy_margin),
                "Circle: {:?}, Test point: {:?}, Expected point: {:?}, Resultant point: {:?}, Accuracy margin: {:?}",
                c,
                test_point,
                expected_point,
                result_point,
                accuracy_margin
            );
        }
    }

    #[test]
    fn circle_intersections() {
        let circle_a = Circle::new((100., 100.), 50.);
        let circle_b = Circle::new((150., 100.), 50.);

        let dist_to_zero = |a: &Point, b: &Point| {
            a.distance(point(0, 0))
                .partial_cmp(&b.distance(point(0, 0)))
                .unwrap()
        };

        let mut intersections = circle_a.intersections(&circle_b);
        let mut expecteds = vec![point(125.0, 143.301), point(125.0, 56.699)];
        intersections.sort_by(dist_to_zero);
        expecteds.sort_by(dist_to_zero);

        // TODO: improve this handling
        let accuracy_margin = 1e-1;
        for (expected, actual) in expecteds.iter().zip(&intersections) {
            assert!(
                point_rel_eq(*expected, *actual, accuracy_margin),
                "Expected: {:?}, Actual: {:?}, Accuracy margin: {:?}, Expected Vec: {:?}, Actual Vec: {:?}",
                expected,
                actual,
                accuracy_margin,
                expecteds,
                intersections,
            );
        }
    }
}
