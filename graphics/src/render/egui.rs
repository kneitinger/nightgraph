use super::RenderResult;
use crate::geometry;
use crate::geometry::WrapsShape;
use egui::{Color32, Pos2, Shape, Stroke};

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);

type Shapes = Vec<Shape>;
pub trait EguiRenderable {
    fn to_shapes(&self) -> RenderResult<Shapes>;
}

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
            .map(|path| Shape::Path {
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
