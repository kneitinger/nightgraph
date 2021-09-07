use euclid::{Angle, Box2D, Point2D, Rotation2D};
use itertools::Itertools;
use num_traits::ToPrimitive;
use svg::node::element::{path::Data, Circle as SvgCircle, Path as SvgPath};
use svg::node::Node;

pub struct PageSpace;

pub type Point = Point2D<f64, PageSpace>;

/// Convenience function to allow making `Point`s quickly
/// from any compatible number type
pub fn point<T: ToPrimitive, U: ToPrimitive>(x: T, y: U) -> Point {
    Point::new(x.to_f64().unwrap(), y.to_f64().unwrap())
}

#[derive(Debug)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Line {
        Self { a, b }
    }

    fn lerp(&self, t: f64) -> Point {
        self.a.lerp(self.b, t)
    }
}

impl Pathable<SvgPath> for Line {
    fn to_points(&self) -> Vec<Point> {
        vec![self.a, self.b]
    }

    fn to_path(&self) -> SvgPath {
        let d = Data::new()
            .move_to(self.a.to_tuple())
            .line_to(self.b.to_tuple());

        SvgPath::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("d", d)
    }

    fn hatch(&self, _spacing: f64, _inset: f64, _angle: f64) -> Vec<Line> {
        vec![]
    }
}

#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Circle {
        Self { center, radius }
    }
}

impl Pathable<SvgCircle> for Circle {
    fn to_points(&self) -> Vec<Point> {
        vec![self.center]
    }

    fn to_path(&self) -> SvgCircle {
        SvgCircle::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("cx", self.center.x)
            .set("cy", self.center.y)
            .set("r", self.radius)
    }

    fn hatch(&self, _spacing: f64, _inset: f64, _angle: f64) -> Vec<Line> {
        vec![]
    }
}
pub struct Bezier {
    a: Point,
    c1: Point,
    c2: Point,
    b: Point,
    steps: u64,
}

impl Bezier {
    pub fn new(a: Point, c1: Point, c2: Point, b: Point) -> Bezier {
        Self {
            a,
            c1,
            c2,
            b,
            steps: 30,
        }
    }

    pub fn new_with_steps(a: Point, c1: Point, c2: Point, b: Point, steps: u64) -> Bezier {
        Self {
            a,
            c1,
            c2,
            b,
            steps,
        }
    }

    pub fn bounds(&self) -> Vec<Line> {
        vec![
            Line::new(self.a, self.c1),
            Line::new(self.c1, self.c2),
            Line::new(self.c2, self.b),
        ]
    }

    pub fn to_lines(&self) -> Vec<Line> {
        self.to_points()
            .iter()
            .tuple_windows()
            .map(|(a, b)| Line { a: *a, b: *b })
            .collect()
    }
}

impl Pathable<SvgPath> for Bezier {
    fn to_points(&self) -> Vec<Point> {
        fn decasteljau(lines: Vec<Line>, t: f64) -> Point {
            let new_lines: Vec<Line> = lines
                .iter()
                .tuple_windows()
                .map(|(la, lb)| Line {
                    a: la.lerp(t),
                    b: lb.lerp(t),
                })
                .collect();

            if new_lines.len() == 1 {
                new_lines[0].lerp(t)
            } else {
                decasteljau(new_lines, t)
            }
        }
        let delta = 1.0 / self.steps as f64;

        (0..=self.steps)
            .map(|n| decasteljau(self.bounds(), 1.0 - (delta * n as f64)))
            .collect()
    }

    fn to_path(&self) -> SvgPath {
        let mut d = Data::new().move_to(self.a.to_tuple());
        d = d.cubic_curve_to((self.c1.to_tuple(), self.c2.to_tuple(), self.b.to_tuple()));

        SvgPath::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("d", d)
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

impl Pathable<SvgPath> for MultiLine {
    fn to_points(&self) -> Vec<Point> {
        self.points.clone()
    }

    fn to_path(&self) -> SvgPath {
        let mut d = Data::new().move_to(self.points[0].to_tuple());
        self.points
            .iter()
            .skip(1)
            .for_each(|p| d = d.clone().line_to(p.to_tuple()));

        SvgPath::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("d", d)
    }

    fn hatch(&self, _spacing: f64, _inset: f64, _angle: f64) -> Vec<Line> {
        vec![]
    }
}

#[derive(Debug)]
pub struct Poly {
    points: Vec<Point>,
    bound_min: Point,
    bound_max: Point,
    centroid: Point,
}

impl Poly {
    pub fn new(points: Vec<Point>) -> Poly {
        let init_point = points.first().unwrap();
        let mut min_x = init_point.x;
        let mut min_y = init_point.y;
        let mut max_x = init_point.x;
        let mut max_y = init_point.y;

        for p in &points {
            if p.x < min_x {
                min_x = p.x;
            }
            if p.x > max_x {
                max_x = p.x;
            }
            if p.y < min_y {
                min_y = p.y;
            }
            if p.y > max_y {
                max_y = p.y;
            }
        }

        let avg_x = points.iter().map(|p| p.x).sum::<f64>() / points.len() as f64;
        let avg_y = points.iter().map(|p| p.y).sum::<f64>() / points.len() as f64;

        Self {
            points,
            bound_min: point(min_x, min_y),
            bound_max: point(max_x, max_y),
            centroid: point(avg_x, avg_y),
        }
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn bounding_coords(&self) -> (Point, Point) {
        (self.bound_min, self.bound_max)
    }

    pub fn new_scaled_to_centroid(&self, t: f64) -> Poly {
        let mut lerped_points = vec![];
        for p in &self.points {
            lerped_points.push(p.lerp(self.centroid, t));
        }
        Poly::new(lerped_points)
    }

    pub fn rotated(&self, angle: f64) -> Poly {
        let r = Rotation2D::new(Angle::degrees(angle));
        Poly::new(self.points.iter().map(|p| r.transform_point(*p)).collect())
    }

    pub fn new_from_points(points: Vec<Point>) -> Option<Poly> {
        fn is_counterclockwise(p: &Point, q: &Point, r: &Point) -> bool {
            (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y) < 0.
        }

        //There must be at least 3 points
        if points.len() < 3 {
            return None;
        }

        let mut hull = vec![];

        //Find the left most point in the polygon
        let (left_most_idx, _) = points
            .iter()
            .enumerate()
            .min_by(|lhs, rhs| lhs.1.x.partial_cmp(&rhs.1.x).unwrap())
            .expect("No left most point");

        let mut p = left_most_idx;

        loop {
            //The left most point must be part of the hull
            hull.push(points[p]);

            let mut q = (p + 1) % points.len();

            for i in 0..points.len() {
                if is_counterclockwise(&points[p], &points[i], &points[q]) {
                    q = i;
                }
            }

            p = q;

            //Break from loop once we reach the first point again
            if p == left_most_idx {
                break;
            }
        }

        Some(Poly::new(hull))
    }

    /// Returns `true` if the point is within the path of the polygon.
    ///
    /// [Reference article](https://wrf.ecse.rpi.edu/Research/Short_Notes/pnpoly.html)
    pub fn contains_point(&self, p: &Point) -> bool {
        let mut crossed = false;

        let mut j = self.points.len() - 1;
        for i in 0..self.points.len() {
            let a = &self.points[i];
            let b = &self.points[j];

            let a_below_p = a.y > p.y;
            let b_below_p = b.y > p.y;
            let only_one_below_p = a_below_p != b_below_p;

            let how_much_more_right_b_is_than_a = b.x - a.x;
            let how_much_below_b_is_than_a = b.y - a.y;
            let how_much_below_p_is_than_a = p.y - a.y;

            /*
             * (a.y > p.y) != (b.y > p.y)
             * &&
             * (p.x <
             *    a.x + (b.x - a.x) * (p.y - a.y) / (b.y - a.y)
             * )
             */

            if only_one_below_p
                && (p.x
                    < a.x
                        + how_much_more_right_b_is_than_a * how_much_below_p_is_than_a
                            / how_much_below_b_is_than_a)
            {
                crossed = !crossed;
            }
            j = i
        }
        crossed
    }
}

impl Pathable<SvgPath> for Poly {
    fn to_points(&self) -> Vec<Point> {
        self.points.clone()
    }

    fn to_path(&self) -> SvgPath {
        let mut d = Data::new().move_to(self.points[0].to_tuple());
        for i in 1..self.points.len() {
            d = d.line_to(self.points[i].to_tuple());
        }
        d = d.close();

        SvgPath::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("d", d)
    }
}

#[derive(Debug)]
pub struct ComplexPoly {
    sub_polys: Vec<Poly>,
    bound_min: Point,
    bound_max: Point,
}

impl ComplexPoly {
    pub fn new(sub_polys: Vec<Poly>) -> ComplexPoly {
        let init_point = sub_polys.first().unwrap().points.first().unwrap();
        let mut min_x = init_point.x;
        let mut min_y = init_point.y;
        let mut max_x = init_point.x;
        let mut max_y = init_point.y;

        for p in sub_polys.iter().map(|poly| &poly.points).flatten() {
            if p.x < min_x {
                min_x = p.x;
            }
            if p.x > max_x {
                max_x = p.x;
            }
            if p.y < min_y {
                min_y = p.y;
            }
            if p.y > max_y {
                max_y = p.y;
            }
        }

        Self {
            sub_polys,
            bound_min: point(min_x, min_y),
            bound_max: point(max_x, max_y),
        }
    }

    pub fn bounding_coords(&self) -> (Point, Point) {
        (self.bound_min, self.bound_max)
    }

    /// Returns `true` if the point is within the path of the polygon.
    ///
    /// [Reference article](https://wrf.ecse.rpi.edu/Research/Short_Notes/pnpoly.html)
    pub fn contains_point(&self, p: Point) -> bool {
        let mut crossed = false;

        for poly in &self.sub_polys {
            let points = poly.to_points();
            let mut j = points.len() - 1;
            for i in 0..points.len() {
                let a = points[i];
                let b = points[j];

                let a_below_p = a.y > p.y;
                let b_below_p = b.y > p.y;
                let only_one_below_p = a_below_p != b_below_p;

                let how_much_more_right_b_is_than_a = b.x - a.x;
                let how_much_below_b_is_than_a = b.y - a.y;
                let how_much_below_p_is_than_a = p.y - a.y;

                /*
                 * (a.y > p.y) != (b.y > p.y)
                 * &&
                 * (p.x <
                 *    a.x + (b.x - a.x) * (p.y - a.y) / (b.y - a.y)
                 * )
                 */

                if only_one_below_p
                    && (p.x
                        < a.x
                            + how_much_more_right_b_is_than_a * how_much_below_p_is_than_a
                                / how_much_below_b_is_than_a)
                {
                    crossed = !crossed;
                }
                j = i
            }
        }
        crossed
    }
}

impl Pathable<SvgPath> for ComplexPoly {
    fn to_points(&self) -> Vec<Point> {
        vec![]
    }

    fn to_path(&self) -> SvgPath {
        let mut d = Data::new();
        for p in &self.sub_polys {
            let points = p.to_points();
            d = d.move_to(points[0].to_tuple());
            for p in points.iter().skip(1) {
                d = d.line_to(p.to_tuple());
            }
            d = d.close();
        }
        SvgPath::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("d", d)
    }
}

/// Represents the ability to be converted to a path, with optional hatch fill.
pub trait Pathable<T: Node> {
    /// Returns the verticies of the line decomposition of the shape
    fn to_points(&self) -> Vec<Point>;
    fn to_path(&self) -> T;
    fn hatch(&self, spacing: f64, inset: f64, angle: f64) -> Vec<Line> {
        let r = Rotation2D::new(Angle::degrees(angle));

        let points: Vec<Point> = self
            .to_points()
            .iter()
            .map(|p| r.transform_point(*p))
            .collect();
        let bb = Box2D::from_points(&points);
        let min_y = bb.min.y;
        let max_y = bb.max.y;

        let mut lines: Vec<Line> = vec![];

        let num_lines = ((max_y - min_y) / spacing) as usize;
        for n_y in 0..num_lines {
            let y = min_y + spacing * n_y as f64;

            let mut j = points.len() - 1;
            let mut x_vals = vec![];
            for i in 0..points.len() {
                let a = points[i];
                let b = points[j];

                if a.y < y && b.y >= y || b.y < y && a.y >= y {
                    x_vals.push(a.x + (y - a.y) / (b.y - a.y) * (b.x - a.x));
                }
                j = i;
            }

            x_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());

            assert!(x_vals.len() % 2 == 0);
            for n in (1..=x_vals.len()).step_by(2) {
                let line = Line::new(
                    r.inverse()
                        .transform_point(Point::new(x_vals[n - 1] + inset, y)),
                    r.inverse()
                        .transform_point(Point::new(x_vals[n] - inset, y)),
                );
                if line.b.x - line.a.x > inset * 2. {
                    lines.push(line);
                }
            }
        }

        lines
    }
}
