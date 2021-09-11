use super::RenderResult;
use crate::geometry;
use crate::geometry::Pointable;
use egui::{Color32, Pos2, Shape, Stroke};

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);

type Shapes = Vec<Shape>;
pub trait EguiRenderable {
    fn to_shapes(&self) -> RenderResult<Shapes>;
}

impl EguiRenderable for geometry::Circle {
    fn to_shapes(&self) -> RenderResult<Shapes> {
        let c = self.center();
        Ok(vec![Shape::Circle {
            center: Pos2::new(c.x as f32, c.y as f32),
            radius: 10.,
            fill: TRANSPARENT,
            // TODO: allow stroke to be set at or before render time
            stroke: Stroke::new(2., WHITE),
        }])
    }
}

impl<T: geometry::Pathable + Pointable> EguiRenderable for T {
    fn to_shapes(&self) -> RenderResult<Shapes> {
        let primary_path = self.to_path()?;
        let paths = primary_path.separate()?;
        let path_points = paths
            .iter()
            .map(|p| p.to_points())
            .collect::<geometry::GeomResult<Vec<Vec<geometry::Point>>>>()?;

        Ok(path_points
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
