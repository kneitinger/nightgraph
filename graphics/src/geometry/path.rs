use super::error::*;
use super::*;
use kurbo::{BezPath, Shape as KurboShape};

enum PathBuildMode {
    Points,
    PointsSmooth,
    Cmds,
    Unknown,
}

pub struct PathBuilder {
    points: Vec<Point>,
    closed: bool,
    cmds: Vec<PathEl>,
    stroke_width: f64,
    precompute: bool,
    mode: PathBuildMode,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            points: vec![],
            closed: false,
            cmds: vec![],
            precompute: false,
            stroke_width: DEFAULT_STROKE_WIDTH,
            mode: PathBuildMode::Unknown,
        }
    }

    pub fn closed(&mut self) -> &mut Self {
        self.closed = true;
        self
    }

    pub fn stroke_width(&mut self, stroke_width: f64) -> &mut Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn precompute(&mut self) -> &mut Self {
        self.precompute = true;
        self
    }

    pub fn points(&mut self, points: &[Point]) -> &mut Self {
        self.points = Vec::from(points);
        self.mode = PathBuildMode::Points;
        self
    }

    pub fn commands(&mut self, cmds: &[PathEl]) -> &mut Self {
        self.cmds = Vec::from(cmds);
        self.mode = PathBuildMode::Cmds;
        self
    }

    pub fn smooth(&mut self) -> &mut Self {
        self.mode = PathBuildMode::PointsSmooth;
        self
    }

    fn bez_from_points_smooth(points: &[Point]) -> GeomResult<BezPath> {
        use std::cmp::Ordering;
        match points.len().cmp(&2) {
            Ordering::Less => Err(GeomError::path_error("path requires at least 2 points")),
            Ordering::Equal => Self::bez_from_points(points, false),
            Ordering::Greater => {
                let xs: Vec<f64> = points.iter().map(|p| p.x).collect();
                let ys: Vec<f64> = points.iter().map(|p| p.y).collect();
                let (cp1_xs, cp2_xs) = smoothing_control_values(&xs);
                let (cp1_ys, cp2_ys) = smoothing_control_values(&ys);
                let mut cmds = vec![PathEl::MoveTo(points[0])];
                for i in 1..points.len() {
                    let c1 = point(cp1_xs[i - 1], cp1_ys[i - 1]);
                    let c2 = point(cp2_xs[i - 1], cp2_ys[i - 1]);
                    cmds.push(PathEl::CurveTo(c1, c2, points[i]));
                }
                Ok(BezPath::from_vec(cmds))
            }
        }
    }

    fn bez_from_points_smooth_closed(points: &[Point]) -> GeomResult<BezPath> {
        use std::cmp::Ordering;
        match points.len().cmp(&2) {
            Ordering::Less => Err(GeomError::path_error("path requires at least 2 points")),
            Ordering::Equal => Self::bez_from_points(points, false),
            Ordering::Greater => {
                let knots = {
                    let mut v = points.to_vec();
                    for &item in points {
                        v.push(item);
                    }
                    v
                };

                let xs: Vec<f64> = knots.iter().map(|p| p.x).collect();
                let ys: Vec<f64> = knots.iter().map(|p| p.y).collect();
                let (cp1_xs, cp2_xs) = smoothing_control_values(&xs);
                let (cp1_ys, cp2_ys) = smoothing_control_values(&ys);
                let mut cmds = vec![PathEl::MoveTo(knots[knots.len() / 2 - 3])];
                let start = knots.len() / 2 - 2;
                let end = knots.len() - 2;
                for i in start..end {
                    let c1 = point(cp1_xs[i - 1], cp1_ys[i - 1]);
                    let c2 = point(cp2_xs[i - 1], cp2_ys[i - 1]);
                    cmds.push(PathEl::CurveTo(c1, c2, knots[i]));
                }
                cmds.push(PathEl::ClosePath);
                Ok(BezPath::from_vec(cmds))
            }
        }
    }

    fn bez_from_points(points: &[Point], closed: bool) -> GeomResult<BezPath> {
        if points.len() < 2 {
            return Err(GeomError::path_error("path requires at least 2 points"));
        }
        let mut cmds = vec![PathEl::MoveTo(points[0])];
        for &p in points.iter().skip(1) {
            cmds.push(PathEl::LineTo(p));
        }
        if closed {
            cmds.push(PathEl::ClosePath);
        }

        Ok(BezPath::from_vec(cmds))
    }

    fn bez_from_commands(cmds: &[PathEl], closed: bool) -> GeomResult<BezPath> {
        match cmds {
            [PathEl::MoveTo(_), _, .., PathEl::ClosePath] => Ok(BezPath::from_vec(Vec::from(cmds))),
            [PathEl::MoveTo(_), _, ..] => {
                if closed {
                    Ok(BezPath::from_vec([cmds, &[PathEl::ClosePath]].concat()))
                } else {
                    Ok(BezPath::from_vec(Vec::from(cmds)))
                }
            }
            [_, ..] => Err(GeomError::path_error(
                "paths must start with a MoveTo command",
            )),
            _ => Err(GeomError::path_error("path requires at least 2 commands")),
        }
    }

    pub fn build(&self) -> GeomResult<Path> {
        let inner = match self.mode {
            PathBuildMode::Cmds => Self::bez_from_commands(&self.cmds, self.closed),
            PathBuildMode::Points => Self::bez_from_points(&self.points, self.closed),
            PathBuildMode::PointsSmooth => {
                if self.closed {
                    Self::bez_from_points_smooth_closed(&self.points)
                } else {
                    Self::bez_from_points_smooth(&self.points)
                }
            }
            PathBuildMode::Unknown => GeomResult::Err(GeomError::path_error("TODO: fill me in")),
        }?;

        let bounding_box = if self.precompute {
            Some(inner.bounding_box())
        } else {
            None
        };

        let stroke_width = self.stroke_width;

        Ok(Path {
            inner,
            bounding_box,
            stroke_width,
        })
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    inner: BezPath,
    bounding_box: Option<kurbo::Rect>,
    stroke_width: f64,
}

impl From<BezPath> for Path {
    fn from(bez_path: BezPath) -> Self {
        Self {
            inner: bez_path,
            bounding_box: None,
            stroke_width: DEFAULT_STROKE_WIDTH,
        }
    }
}

impl Path {
    pub fn new(origin: Point, cmd: PathEl) -> Self {
        let mut inner = BezPath::new();
        inner.move_to(origin);
        inner.push(cmd);
        Self {
            inner,
            bounding_box: None,
            stroke_width: DEFAULT_STROKE_WIDTH,
        }
    }

    pub fn from_points(points: &[Point]) -> Self {
        let mut cmds = vec![PathEl::MoveTo(points[0])];
        for &p in points.iter().skip(1) {
            cmds.push(PathEl::LineTo(p));
        }

        Path::from_commands(&cmds).unwrap()
    }

    pub fn from_commands(commands: &[PathEl]) -> GeomResult<Self> {
        match commands {
            [PathEl::MoveTo(_), _, ..] => Ok(Self {
                inner: BezPath::from_vec(Vec::from(commands)),
                bounding_box: None,
                stroke_width: DEFAULT_STROKE_WIDTH,
            }),
            [_, ..] => Err(GeomError::path_error(
                "paths must start with a MoveTo command",
            )),
            _ => Err(GeomError::path_error("path requires at least 2 commands")),
        }
    }

    pub fn from_points_smooth(knots: &[Point]) -> Self {
        if knots.len() == 2 {
            return Path::from_points(knots);
        }

        let xs: Vec<f64> = knots.iter().map(|p| p.x).collect();
        let ys: Vec<f64> = knots.iter().map(|p| p.y).collect();
        let (cp1_xs, cp2_xs) = smoothing_control_values(&xs);
        let (cp1_ys, cp2_ys) = smoothing_control_values(&ys);
        let mut cmds = vec![PathEl::MoveTo(knots[0])];
        for i in 1..knots.len() {
            let c1 = point(cp1_xs[i - 1], cp1_ys[i - 1]);
            let c2 = point(cp2_xs[i - 1], cp2_ys[i - 1]);
            cmds.push(PathEl::CurveTo(c1, c2, knots[i]));
        }
        Path::from_commands(&cmds).unwrap()
    }

    pub fn from_points_smooth_closed(knots: &[Point]) -> Self {
        if knots.len() == 2 {
            return Path::from_points(knots);
        }

        let knots = {
            let mut v = knots.to_vec();
            for &item in knots {
                v.push(item);
            }
            v
        };

        let xs: Vec<f64> = knots.iter().map(|p| p.x).collect();
        let ys: Vec<f64> = knots.iter().map(|p| p.y).collect();
        let (cp1_xs, cp2_xs) = smoothing_control_values(&xs);
        let (cp1_ys, cp2_ys) = smoothing_control_values(&ys);
        let mut cmds = vec![PathEl::MoveTo(knots[knots.len() / 2 - 3])];
        let start = knots.len() / 2 - 2;
        let end = knots.len() - 2;
        for i in start..end {
            let c1 = point(cp1_xs[i - 1], cp1_ys[i - 1]);
            let c2 = point(cp2_xs[i - 1], cp2_ys[i - 1]);
            cmds.push(PathEl::CurveTo(c1, c2, knots[i]));
        }
        cmds.push(PathEl::ClosePath);
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
            bounding_box: self.bounding_box,
            stroke_width: self.stroke_width,
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
    fn stroke(&self) -> f64 {
        self.stroke_width
    }
    fn as_shape(&self) -> Shape {
        Shape::Path(self.clone())
    }
    fn perimeter(&self) -> f64 {
        self.inner.perimeter(DEFAULT_ACCURACY)
    }
    fn bounding_box(&self) -> kurbo::Rect {
        match self.bounding_box {
            Some(bb) => bb,
            None => self.inner.bounding_box(),
        }
    }
    fn contains(&self, p: Point) -> bool {
        self.inner.contains(p)
    }
    fn area(&self) -> f64 {
        self.inner.area()
    }
}

// From http://www.particleincell.com/2012/bezier-splines/
// Permission with attribution granted in example's source:
// http://www.particleincell.com/wp-content/uploads/2012/06/circles.svg
#[allow(clippy::many_single_char_names)]
fn smoothing_control_values(values: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = values.len() - 1;
    let mut p1 = vec![0.; n];
    let mut p2 = vec![0.; n];

    /*rhs vector*/
    let mut a = vec![0.; n];
    let mut b = vec![0.; n];
    let mut c = vec![0.; n];
    let mut r = vec![0.; n];

    // Left segment
    a[0] = 0.;
    b[0] = 2.;
    c[0] = 1.;
    r[0] = values[0] + 2. * values[1];

    // Middle segments
    for i in 1..n - 1 {
        a[i] = 1.;
        b[i] = 4.;
        c[i] = 1.;
        r[i] = 4. * values[i] + 2. * values[i + 1];
    }

    // Right segment
    a[n - 1] = 2.;
    b[n - 1] = 7.;
    c[n - 1] = 0.;
    r[n - 1] = 8. * values[n - 1] + values[n];

    // Solves Ax=b with the Thomas algorithm (from Wikipedia)
    for i in 1..n {
        let m = a[i] / b[i - 1];
        b[i] -= m * c[i - 1];
        r[i] -= m * r[i - 1];
    }

    p1[n - 1] = r[n - 1] / b[n - 1];
    for i in (0..=n - 2).rev() {
        p1[i] = (r[i] - c[i] * p1[i + 1]) / b[i];
    }

    // Compute 2nd control points now that 1st are obtained
    for i in 0..n - 1 {
        p2[i] = 2. * values[i + 1] - p1[i + 1];
    }

    p2[n - 1] = 0.5 * (values[n] + p1[n - 1]);

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::point;
    #[test]
    fn path_builder_closed() {
        let points = vec![point(0, 0), point(2, 0), point(2, 2), point(0, 2)];
        let built_path = PathBuilder::new().points(&points).closed().build().unwrap();
        let built_path_no_close = PathBuilder::new().points(&points).build().unwrap();

        assert!((built_path.area() - 4.0).abs() < f64::EPSILON);
        assert!((built_path.perimeter() - 8.0).abs() < f64::EPSILON);
        assert!(built_path.perimeter() > built_path_no_close.perimeter(),);
    }

    #[test]
    fn path_builder_precompute() {
        let points = vec![point(0, 0), point(2, 0), point(2, 2), point(0, 2)];
        let built_path = PathBuilder::new()
            .points(&points)
            .precompute()
            .build()
            .unwrap();
        let built_path_no_precompute = PathBuilder::new().points(&points).build().unwrap();

        assert!(matches!(built_path.bounding_box, Some(_bb)));
        assert!(matches!(built_path_no_precompute.bounding_box, None));
    }
}
