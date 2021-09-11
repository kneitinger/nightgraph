use super::RenderResult;
use crate::geometry;
use crate::geometry::PathCommand;
use svg::node::element::{path::Data, Circle as SvgCircle, Path as SvgPath};
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

impl<T: geometry::Pathable> SvgRenderable<SvgPath> for T {
    fn to_svg(&self) -> RenderResult<SvgPath> {
        let mut d = Data::new();
        let path = self.to_path()?;

        for cmd in path.commands() {
            d = match cmd {
                PathCommand::MoveTo(p) => d.move_to(p.to_tuple()),
                PathCommand::LineTo(p) => d.line_to(p.to_tuple()),
                PathCommand::QuadTo(c, p) => d.quadratic_curve_to((c.to_tuple(), p.to_tuple())),
                PathCommand::CurveTo(c1, c2, p) => {
                    d.cubic_curve_to((c1.to_tuple(), c2.to_tuple(), p.to_tuple()))
                }
                PathCommand::Close => d.close(),
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
