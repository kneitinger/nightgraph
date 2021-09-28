use super::error::*;
use super::*;
use kurbo::{BezPath, Shape as KurboShape};

#[derive(Clone, Debug)]
pub struct Path {
    inner: BezPath,
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

    pub fn from_points(points: &[Point]) -> Self {
        let mut cmds = vec![];
        cmds.push(PathEl::MoveTo(points[0]));
        for i in 1..points.len() {
            cmds.push(PathEl::LineTo(points[i]));
        }

        Path::from_commands(&cmds).unwrap()
    }

    pub fn from_commands(commands: &[PathEl]) -> GeomResult<Self> {
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

    pub fn from_points_smooth(knots: &[Point]) -> Self {
        Self::from_points_smooth_internal(knots, false)
    }

    pub fn from_points_smooth_closed(knots: &[Point]) -> Self {
        Self::from_points_smooth_internal(knots, true)
    }

    fn from_points_smooth_internal(knots: &[Point], closed: bool) -> Self {
        // From http://www.particleincell.com/2012/bezier-splines/
        // Permission with attribution granted in example's source:
        // http://www.particleincell.com/wp-content/uploads/2012/06/circles.svg
        fn control_points(knots: &[f64]) -> (Vec<f64>, Vec<f64>) {
            let n = knots.len() - 1;
            let mut p1 = vec![0.; n];
            let mut p2 = vec![0.; n];

            /*rhs vector*/
            let mut a = vec![0.; n];
            let mut b = vec![0.; n];
            let mut c = vec![0.; n];
            let mut r = vec![0.; n];

            /*left most segment*/
            a[0] = 0.;
            b[0] = 2.;
            c[0] = 1.;
            r[0] = knots[0] + 2. * knots[1];

            for i in 1..n - 1 {
                a[i] = 1.;
                b[i] = 4.;
                c[i] = 1.;
                r[i] = 4. * knots[i] + 2. * knots[i + 1];
            }

            /*right segment*/
            a[n - 1] = 2.;
            b[n - 1] = 7.;
            c[n - 1] = 0.;
            r[n - 1] = 8. * knots[n - 1] + knots[n];

            /*solves Ax=b with the Thomas algorithm (from Wikipedia)*/
            for i in 1..n {
                let m = a[i] / b[i - 1];
                b[i] = b[i] - m * c[i - 1];
                r[i] = r[i] - m * r[i - 1];
            }

            p1[n - 1] = r[n - 1] / b[n - 1];
            for i in (0..=n - 2).rev() {
                p1[i] = (r[i] - c[i] * p1[i + 1]) / b[i];
            }

            /*we have p1, now compute p2*/
            for i in 0..n - 1 {
                p2[i] = 2. * knots[i + 1] - p1[i + 1];
            }

            p2[n - 1] = 0.5 * (knots[n] + p1[n - 1]);

            (p1, p2)
        }

        if knots.len() == 2 {
            return Path::from_points(&knots);
        }

        let knots = if closed {
            let mut v = knots.to_vec();
            let first = v[0];
            let last = v[v.len() - 1];

            let avg = avg_point(first, last);
            let f_avg = avg_point(avg, first);
            let l_avg = avg_point(avg, last);

            v.insert(0, f_avg);
            v.insert(0, avg);

            v.push(l_avg);
            v.push(avg);
            v.push(first);
            v
        } else {
            knots.to_vec()
        };

        let xs: Vec<f64> = knots.iter().map(|p| p.x).collect();
        let ys: Vec<f64> = knots.iter().map(|p| p.y).collect();
        let (cp1_xs, cp2_xs) = control_points(&xs);
        let (cp1_ys, cp2_ys) = control_points(&ys);

        let mut cmds = vec![];
        cmds.push(PathEl::MoveTo(knots[0]));
        let end = if closed { knots.len() - 1 } else { knots.len() };
        for i in 2..end {
            let c1 = point(cp1_xs[i - 1], cp1_ys[i - 1]);
            let c2 = point(cp2_xs[i - 1], cp2_ys[i - 1]);
            cmds.push(PathEl::CurveTo(c1, c2, knots[i]));
        }
        if closed {
            cmds.push(PathEl::ClosePath);
        }

        Path::from_commands(&cmds).unwrap()
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
                        paths.push(Path::from_commands(path_cmds.as_slice())?);
                        path_cmds.clear();
                        path_cmds.push(cmd);
                    }
                }
                _ => path_cmds.push(cmd),
            }
        }
        paths.push(Path::from_commands(path_cmds.as_slice())?);

        Ok(paths)
    }

    pub fn inner(&self) -> &BezPath {
        &self.inner
    }

    pub fn translate(&self, translation: Vec2) -> Self {
        let ts = kurbo::TranslateScale::new(translation, 1.0);
        Self {
            inner: ts * self.inner.clone(),
        }
    }
}

impl Shaped for Path {
    fn to_path(&self) -> Path {
        self.clone()
    }
    fn as_bezpath(&self) -> BezPath {
        self.inner.clone()
    }
    fn as_shape(&self) -> Shape {
        Shape::Path(self.clone())
    }
    fn perimeter(&self) -> f64 {
        self.inner.perimeter(DEFAULT_ACCURACY)
    }
    fn bounding_box(&self) -> kurbo::Rect {
        self.inner.bounding_box()
    }
    fn contains(&self, p: Point) -> bool {
        self.inner.contains(p)
    }
    fn area(&self) -> f64 {
        self.inner.area()
    }
}
