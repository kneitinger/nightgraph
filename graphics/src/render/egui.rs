use crate::canvas::*;
use crate::geometry::{Circle, Point, Shape};
use egui::{Color32, Pos2, Shape as EguiShape, Stroke};

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

pub trait EguiRenderer {
    fn render_egui(&self) -> (egui::Vec2, Vec<EguiShape>);
}

impl EguiRenderer for Canvas {
    fn render_egui(&self) -> (egui::Vec2, Vec<EguiShape>) {
        (
            egui::Vec2::new(self.width() as f32, self.height() as f32),
            self.render(),
        )
    }
}

pub trait EguiRenderable {
    fn render(&self) -> Vec<EguiShape>;
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
    fn render(&self) -> Vec<EguiShape> {
        self.elements()
            .iter()
            .map(|e| e.render())
            .flatten()
            .collect()
    }
}

impl EguiRenderable for Circle {
    fn render(&self) -> Vec<EguiShape> {
        let c = self.inner().center;
        vec![EguiShape::circle_stroke(
            Pos2::new(c.x as f32, c.y as f32),
            self.inner().radius as f32,
            // TODO: allow stroke to be set at or before render time
            Stroke::new(2., WHITE),
        )]
    }
}

impl EguiRenderable for Shape {
    fn render(&self) -> Vec<EguiShape> {
        match self {
            Self::Circle(c) => c.render(),
            _ => {
                fn p(point: &Point) -> Pos2 {
                    Pos2::new(point.x as f32, point.y as f32)
                }
                let lines = self.to_lines();

                lines
                    .iter()
                    .map(|line| EguiShape::LineSegment {
                        points: [p(&line.p0()), p(&line.p1())],
                        stroke: Stroke::new(2., WHITE),
                    })
                    .collect()
            }
        }
    }
}
