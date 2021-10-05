use eframe::{
    egui,
    egui::{Color32, Painter, Pos2, Shape as EguiShape, Vec2},
    egui::{FontDefinitions, FontFamily, Style},
    epi,
};
use nightgraphics::render::EguiRenderer;
use serde::{Deserialize, Serialize};
use sketches::Sketch;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct NightgraphApp {
    // Temporarily opt out of state persistence on drawing until the sketch
    // and associated info is actually stored in the app state
    #[serde(skip)]
    drawing: Drawing,
}

impl Default for NightgraphApp {
    fn default() -> Self {
        Self {
            drawing: Drawing::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Drawing {
    #[serde(skip)]
    shapes: Vec<egui::Shape>,

    sketch_rect: egui::Rect,
    translation: Vec2,
    zoom: f32,
    init: bool,
    draw_debug_geom: bool,
    draw_page_outline: bool,
}

impl Default for Drawing {
    fn default() -> Self {
        let (sketch_size, shapes) = Sketch::default().exec().unwrap().render_egui();
        let sketch_rect = egui::Rect::from_min_size(Pos2::ZERO, sketch_size);
        Self {
            shapes,
            sketch_rect,
            translation: Vec2::new(0., 0.),
            zoom: 1.,
            init: false,
            draw_debug_geom: false,
            draw_page_outline: false,
        }
    }
}

impl Drawing {
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

impl epi::App for NightgraphApp {
    fn name(&self) -> &str {
        "nightgraph ui"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Jost*".to_owned(),
            std::borrow::Cow::Borrowed(include_bytes!("../assets/Jost-400-Book.otf")),
        );

        // Place font at the hightest priority for proportional
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Jost*".to_owned());

        // Place font at the lowest priority for monospace
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("Jost*".to_owned());

        ctx.set_fonts(fonts);

        let style = Style {
            visuals: egui::Visuals::light(),
            ..Default::default()
        };
        ctx.set_style(style);
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("nightgraph ui");
                egui::warn_if_debug_build(ui);
                ui.checkbox(&mut self.drawing.draw_debug_geom, "Draw debug geometry");
                ui.checkbox(&mut self.drawing.draw_page_outline, "Draw page outline");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                self.drawing.ui_content(ui);
            });
        });
    }
}
