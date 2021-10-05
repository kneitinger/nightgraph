use eframe::{
    egui,
    egui::{Color32, Painter, Pos2, Shape as EguiShape, Vec2},
};

use nightgraphics::render::EguiRenderer;
use serde::{Deserialize, Serialize};
use sketches::SketchList;

#[derive(Deserialize, Serialize)]
pub struct Drawing {
    #[serde(skip)]
    shapes: Vec<egui::Shape>,

    sketch_rect: egui::Rect,
    translation: Vec2,
    zoom: f32,
    init: bool,
    pub draw_debug_geom: bool,
    pub draw_page_outline: bool,
    pub bg_color: Color32,
}

/*
pub struct SketchData {
    sketch: SketchList,
    sketch_rect: egui::Rect,
}
*/

impl Default for Drawing {
    fn default() -> Self {
        let (sketch_size, shapes) = SketchList::default().exec().unwrap().render_egui();
        let sketch_rect = egui::Rect::from_min_size(Pos2::ZERO, sketch_size);
        Self {
            shapes,
            sketch_rect,
            translation: Vec2::new(0., 0.),
            zoom: 1.,
            init: false,
            draw_debug_geom: false,
            draw_page_outline: false,
            bg_color: Color32::from_rgb(30, 30, 30),
        }
    }
}

impl Drawing {
    pub fn rerender(&mut self, render_data: (Vec2, Vec<EguiShape>)) {
        let (sketch_size, shapes) = render_data;
        self.shapes = shapes.to_owned();
        self.sketch_rect = egui::Rect::from_min_size(Pos2::ZERO, sketch_size);
    }
    fn translate_scale(&mut self, transformation: egui::math::RectTransform) -> Vec<EguiShape> {
        self.shapes
            .iter()
            .map(|shape| match shape {
                EguiShape::Circle {
                    center,
                    radius,
                    fill: _,
                    stroke,
                } => EguiShape::circle_stroke(
                    transformation * *center,
                    radius * transformation.scale().x,
                    *stroke,
                ),
                EguiShape::LineSegment { points, stroke } => EguiShape::line_segment(
                    [transformation * points[0], transformation * points[1]],
                    *stroke,
                ),
                EguiShape::Rect {
                    rect,
                    corner_radius,
                    fill: _,
                    stroke,
                } => EguiShape::rect_stroke(
                    transformation.transform_rect(*rect),
                    *corner_radius,
                    *stroke,
                ),
                _ => EguiShape::Noop,
            })
            .collect()
    }
    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        fn circ(painter: &Painter, center: Pos2, radius: f32, color: Color32) {
            painter.add(EguiShape::circle_stroke(
                center,
                radius,
                egui::Stroke::new(5., color),
            ));
        }
        let (response, painter) = ui.allocate_painter(
            ui.available_size_before_wrap_finite(),
            egui::Sense::hover().union(egui::Sense::drag()),
        );

        let phy_rect = response.rect;

        if self.draw_debug_geom {
            circ(&painter, phy_rect.min, 8., Color32::LIGHT_BLUE);
            circ(&painter, phy_rect.center(), 8., Color32::LIGHT_BLUE);
        }

        if !self.init {
            self.translation = self.sketch_rect.center() - phy_rect.center();
            self.zoom = phy_rect.height() / self.sketch_rect.height();
            self.zoom *= 0.9;
            self.init = true;
        }
        if response.drag_delta() != Vec2::ZERO {
            // Dividing by self.zoom ensures that the move amount corresponds
            // with the mouse movement to the user
            self.translation -= response.drag_delta() / self.zoom;
        }

        let to_screen = egui::math::RectTransform::from_to(
            egui::Rect::from_center_size(
                phy_rect.center() + self.translation,
                phy_rect.size() / self.zoom,
            ),
            phy_rect,
        );

        let scroll_delta = ui.input().scroll_delta.y;
        let mouse_pos = ui.input().pointer.interact_pos();
        if scroll_delta != 0.0 && ui.rect_contains_pointer(phy_rect) {
            if let Some(_pos) = mouse_pos {
                let scroll_adj = if scroll_delta > 0. { 1.1 } else { 0.9 };
                self.zoom *= scroll_adj;
            }
        }

        let log_rect = to_screen.transform_rect(self.sketch_rect);
        if self.draw_debug_geom {
            circ(&painter, log_rect.min, 10., Color32::GREEN);
            circ(&painter, log_rect.center(), 10., Color32::GREEN);
        }
        if self.draw_page_outline {
            painter.add(EguiShape::rect_stroke(
                to_screen.transform_rect(self.sketch_rect),
                0.,
                egui::Stroke::new(2., Color32::WHITE),
            ));
        }

        painter.extend(self.translate_scale(to_screen));
    }
}
