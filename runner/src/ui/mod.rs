use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiSettings};
use nightgraphics::geometry::{point, Circle, Text};
use nightgraphics::render::egui::EguiRenderable;

#[derive(Default)]
pub struct UiState {
    canvas: Canvas,
}

pub fn update_ui_scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut toggle_scale_factor: Local<Option<bool>>,
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
        *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        if let Some(window) = windows.get_primary() {
            let scale_factor = if toggle_scale_factor.unwrap() {
                1.2
            } else {
                1.2 / window.scale_factor()
            };
            egui_settings.scale_factor = scale_factor;
        }
    }
}

struct Canvas {
    shapes: Vec<egui::Shape>,
}

impl Default for Canvas {
    fn default() -> Self {
        let circ = Circle::new(point(200, 200), 80.);
        let mut text = Text::default();
        text.set_size(100.);
        text.set_origin(point(200, 300));
        Self {
            shapes: vec![circ.to_shape(), text.to_shape()],
        }
    }
}

impl Canvas {
    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        let (_response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::hover());

        for shape in &self.shapes {
            painter.add(shape.clone());
        }
    }
}

pub fn ui_nightgraph(egui_ctx: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Nightgraph UI");
            egui::warn_if_debug_build(ui);
        });

    egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            ui_state.canvas.ui_content(ui);
        });
    });
}
