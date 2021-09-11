use crate::geometry;
use egui::{Color32, Pos2, Shape, Stroke};

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);
pub trait EguiRenderable {
    fn to_shape(&self) -> Shape;
}

impl EguiRenderable for geometry::Circle {
    fn to_shape(&self) -> Shape {
        let c = self.center();
        Shape::Circle {
            center: Pos2::new(c.x as f32, c.y as f32),
            radius: 10.,
            fill: TRANSPARENT,
            // TODO: allow stroke to be set at or before render time
            stroke: Stroke::new(2., WHITE),
        }
    }
}

impl<T: geometry::Pathable + geometry::Pointable> EguiRenderable for T {
    // TODO: currently doesn't handle multiple shapes per path
    fn to_shape(&self) -> Shape {
        Shape::Path {
            points: self
                .to_points()
                .iter()
                .map(|p| Pos2::new(p.x as f32, p.y as f32))
                .collect(),
            closed: false,
            fill: TRANSPARENT,
            stroke: Stroke::new(2., WHITE),
        }
    }
}
