use crate::canvas::*;
use crate::geometry::{Circle, PathEl, Point, Shape};
use svg::node::element::{path::Data, Circle as SvgCircle, Path as SvgPath};
use svg::Document;

pub trait Renderer {
    fn render_svg(&self) {}
}

impl Renderer for Canvas {
    fn render_svg(&self) {
        let doc = Document::new()
            .set("width", self.width())
            .set("height", self.height());
        let rendered_doc = self.render(doc);
        svg::save("image.svg".to_string(), &rendered_doc).expect("Unable to save SVG");
    }
}

pub trait SvgRenderable {
    fn render(&self, doc: Document) -> Document;
}

impl SvgRenderable for CanvasElement {
    fn render(&self, doc: Document) -> Document {
        match self {
            Self::Canvas(c) => c.render(doc),
            Self::Shape(s) => s.render(doc),
        }
    }
}

impl SvgRenderable for Canvas {
    fn render(&self, doc: Document) -> Document {
        // [Res<Vec>, Res<Vec>]
        self.elements().iter().fold(doc, |acc, c| c.render(acc))
    }
}

impl SvgRenderable for Circle {
    fn render(&self, doc: Document) -> Document {
        let c = SvgCircle::new()
            .set("fill", "none")
            // TODO: allow stroke to be set at or before render time
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("cx", self.center().x)
            .set("cy", self.center().y)
            .set("r", self.radius());
        doc.add(c)
    }
}

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
    fn render(&self, doc: Document) -> Document {
        match self {
            Self::Circle(c) => c.render(doc),
            _ => {
                fn t(point: &Point) -> (f64, f64) {
                    (point.x, point.y)
                }
                let mut d = Data::new();
                let path = self.to_path();

                for cmd in path.inner().elements() {
                    d = match cmd {
                        PathEl::MoveTo(p) => d.move_to(t(p)),
                        PathEl::LineTo(p) => d.line_to(t(p)),
                        PathEl::QuadTo(c, p) => d.quadratic_curve_to((t(c), t(p))),
                        PathEl::CurveTo(c1, c2, p) => d.cubic_curve_to((t(c1), t(c2), t(p))),
                        PathEl::ClosePath => d.close(),
                    }
                }

                doc.add(
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
    }
}
