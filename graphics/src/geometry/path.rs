use super::error::*;
use super::{PathEl, Point, Shape, Shaped, Vec2, DEFAULT_ACCURACY};
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
