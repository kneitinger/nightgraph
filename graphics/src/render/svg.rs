use super::RenderResult;
use crate::geometry;
use crate::geometry::PathEl;
use crate::geometry::WrapsShape;
use svg::node::element::{path::Data, Circle as SvgCircle, Line as SvgLine, Path as SvgPath};
use svg::node::Node;

pub trait SvgRenderable<T: Node> {
    fn to_svg(&self) -> RenderResult<T>;
}

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

//impl<T: geometry::WrapsShape<Inner = kurbo::BezPath>> SvgRenderable<SvgPath> for T
impl<T: geometry::WrapsBez + geometry::WrapsShape<kurbo::BezPath>> SvgRenderable<SvgPath> for T {
    //impl<T: geometry::Pathable> SvgRenderable<SvgPath> for T {
    fn to_svg(&self) -> RenderResult<SvgPath> {
        fn t(point: &geometry::Point) -> (f64, f64) {
            (point.x, point.y)
        }
        let mut d = Data::new();
        let path: kurbo::BezPath = self.inner();

        for cmd in path.elements() {
            d = match cmd {
                PathEl::MoveTo(p) => d.move_to(t(p)),
                PathEl::LineTo(p) => d.line_to(t(p)),
                PathEl::QuadTo(c, p) => d.quadratic_curve_to((t(c), t(p))),
                PathEl::CurveTo(c1, c2, p) => d.cubic_curve_to((t(c1), t(c2), t(p))),
                PathEl::ClosePath => d.close(),
            }
        }

        Ok(SvgPath::new()
            .set("fill", "none")
            // TODO: allow stroke to be set at or before render time
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("fill-rule", "evenodd")
            .set("d", d))
    }
}
