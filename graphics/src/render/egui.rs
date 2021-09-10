use crate::geometry;
use crate::geometry::PathCommand;
use egui::{Color32, Pos2, Shape, Stroke};

const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
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
            stroke: Stroke::new(2., BLACK),
        }
    }
}

impl<T: geometry::Pathable> EguiRenderable for T {
    fn to_shape(&self) -> Shape {
        Shape::Noop
    }
}
