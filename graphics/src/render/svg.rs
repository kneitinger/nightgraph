use super::RenderResult;
use crate::canvas::Canvas;
use crate::canvas::CanvasElement;
use crate::geometry;
use crate::geometry::PathEl;
use crate::geometry::Shape;
use crate::geometry::Shaped;
use svg::node::element::{path::Data, Path as SvgPath};

pub enum Svg {
    Path(SvgPath),
}

impl From<SvgPath> for Svg {
    fn from(p: SvgPath) -> Self {
        Self::Path(p)
    }
}

impl Svg {
    /*
    fn node<T>(&self) -> T {
        match self {
            Self::Path(p) => p,
        }
    }
    */
}

pub trait SvgRenderable {
    fn to_svg(&self) -> Svg;
    fn to_svgs(&self) -> Vec<Svg> {
        vec![self.to_svg()]
    }
}

pub trait SvgRenderableGroup {
    fn to_svgs(&self) -> Vec<Svg>;
}

impl SvgRenderableGroup for CanvasElement {
    fn to_svgs(&self) -> Vec<Svg> {
        match self {
            Self::Canvas(c) => c.to_svgs(),
            Self::Shape(s) => s.to_svgs(),
        }
    }
}

impl SvgRenderableGroup for Canvas {
    fn to_svgs(&self) -> Vec<Svg> {
        // [Res<Vec>, Res<Vec>]
        self.elements()
            .iter()
            .map(|c| c.to_svgs())
            .flatten()
            .collect::<Vec<_>>()
    }
}

/*
impl SvgRenderable<SvgCircle> for geometry::Circle {
    fn to_svg(&self) -> RenderResult<SvgCircle> {
        Ok(SvgCircle::new()
            .set("fill", "none")
            // TODO: allow stroke to be set at or before render time
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("cx", self.center().x)
            .set("cy", self.center().y)
            .set("r", self.radius()))
    }
}
*/

/*
impl SvgRenderable<SvgLine> for geometry::Line {
    fn to_svg(&self) -> RenderResult<SvgLine> {
        let p1 = self.inner().p0;
        let p2 = self.inner().p1;
        Ok(SvgLine::new()
            .set("fill", "none")
            // TODO: allow stroke to be set at or before render time
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("x1", p1.x)
            .set("y1", p1.y)
            .set("x2", p2.x)
            .set("y2", p2.y))
    }
}
*/

//impl<T: geometry::WrapsShape<Inner = kurbo::BezPath>> SvgRenderable<SvgPath> for T
impl SvgRenderable for Shape {
    //impl<T: geometry::Pathable> SvgRenderable<SvgPath> for T {
    fn to_svg(&self) -> Svg {
        fn t(point: &geometry::Point) -> (f64, f64) {
            (point.x, point.y)
        }
        let mut d = Data::new();
        let pathed = self.to_path();
        let path: &kurbo::BezPath = pathed.inner();

        for cmd in path.elements() {
            d = match cmd {
                PathEl::MoveTo(p) => d.move_to(t(p)),
                PathEl::LineTo(p) => d.line_to(t(p)),
                PathEl::QuadTo(c, p) => d.quadratic_curve_to((t(c), t(p))),
                PathEl::CurveTo(c1, c2, p) => d.cubic_curve_to((t(c1), t(c2), t(p))),
                PathEl::ClosePath => d.close(),
            }
        }

        Svg::from(
            SvgPath::new()
                .set("fill", "none")
                // TODO: allow stroke to be set at or before render time
                .set("stroke", "black")
                .set("stroke-width", "0.5mm")
                .set("fill-rule", "evenodd")
                .set("d", d),
        )
    }
}
