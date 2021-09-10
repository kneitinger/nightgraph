use super::{Path, Pathable, Point};
#[derive(Debug)]
pub struct Poly {
    points: Vec<Point>,
}

impl Poly {
    pub fn new(points: Vec<Point>) -> Poly {
        Self { points }
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }

    /*
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
        */
}

impl Pathable for Poly {
    fn to_path(&self) -> Path {
        let mut path = Path::new();
        path.move_to(self.points[0]);
        for &p in self.points.iter().skip(1) {
            path.line_to(p);
        }
        path.close();
        path
    }
}

#[derive(Debug)]
pub struct ComplexPoly {
    sub_polys: Vec<Poly>,
}

impl ComplexPoly {
    pub fn new(sub_polys: Vec<Poly>) -> ComplexPoly {
        Self { sub_polys }
    }

    /*
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
        */
}

impl Pathable for ComplexPoly {
    fn to_path(&self) -> Path {
        let mut path = Path::new();
        for p in &self.sub_polys {
            let points = p.points();
            path.move_to(points[0]);
            for &p in points.iter().skip(1) {
                path.line_to(p);
            }
            path.close();
        }
        path
    }
}
