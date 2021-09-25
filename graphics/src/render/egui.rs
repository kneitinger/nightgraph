use crate::canvas::*;
use crate::geometry::{Circle, Shape};
use egui::{Color32, Pos2, Shape as EguiShape, Stroke};

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);

type Shapes = Vec<EguiShape>;

/*
impl EguiRenderable for geometry::Circle {
    //impl<U: kurbo::Shape> EguiRenderable for dyn geometry::WrapsShape {
    fn to_shapes(&self) -> RenderResult<Shapes> {
        let c = self.inner().center;
        Ok(vec![Shape::circle_stroke(
            Pos2::new(c.x as f32, c.y as f32),
            self.inner().radius as f32,
            // TODO: allow stroke to be set at or before render time
            Stroke::new(2., WHITE),
        )])
    }
}

impl<T: geometry::WrapsShape> EguiRenderable for T {
    //impl<U: kurbo::Shape> EguiRenderable for dyn geometry::WrapsShape {
    fn to_shapes(&self) -> RenderResult<Shapes> {
        let point_groups = self.to_points();

        Ok(point_groups
            .iter()
            .map(|path| EguiShape::Path {
                points: path
                    .iter()
                    .map(|p| Pos2::new(p.x as f32, p.y as f32))
                    .collect(),
                closed: false,
                fill: TRANSPARENT,
                // TODO: allow stroke to be set at or before render time
                stroke: Stroke::new(2., WHITE),
            })
            .collect())
    }
}
*/

pub trait EguiRenderer {
    fn render_egui(&self);
}

impl EguiRenderer for Canvas {
    fn render_egui(&self) {
        self.render();
    }
}

pub trait EguiRenderable {
    fn render(&self) -> Shapes;
}

impl EguiRenderable for CanvasElement {
    fn render(&self) -> Vec<EguiShape> {
        match self {
            Self::Canvas(c) => c.render(),
            Self::Shape(s) => s.render(),
        }
    }
}

impl EguiRenderable for Canvas {
    fn render(&self) -> Shapes {
        // [Res<Vec>, Res<Vec>]
        //self.elements().iter().fold(doc, |acc, c| c.render(acc))
        vec![]
    }
}

impl EguiRenderable for Circle {
    fn render(&self) -> Shapes {
        vec![]
        /*
        let c = SvgCircle::new()
            .set("fill", "none")
            // TODO: allow stroke to be set at or before render time
            .set("stroke", "black")
            .set("stroke-width", "0.5mm")
            .set("cx", self.center().x)
            .set("cy", self.center().y)
            .set("r", self.radius());
        doc.add(c)
        */
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
impl EguiRenderable for Shape {
    fn render(&self) -> Shapes {
        vec![]
        /*
        match self {
            Self::Circle(c) => c.render(),
            _ => {
                fn t(point: &Point) -> (f64, f64) {
                    (point.x, point.y)
                }
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
        */
    }
}
