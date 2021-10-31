use eframe::{
    egui,
    egui::{FontDefinitions, FontFamily, Style},
    epi,
};
use nightgraphics::render::EguiRenderer;
use nightsketch::{ParamKind, ParamMetadata, ParamRange, SketchList};

mod drawing;
use drawing::Drawing;

mod sketch_control;
use sketch_control::*;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct NightgraphApp {
    // Temporarily opt out of state persistence on drawing until the sketch
    // and associated info is actually stored in the app state
    #[serde(skip)]
    drawing: Drawing,

    #[serde(skip)]
    sketch_control: SketchControl,

    ui_scale: Option<f32>,
}

impl Default for NightgraphApp {
    fn default() -> Self {
        Self {
            sketch_control: SketchControl::default(),
            drawing: Drawing::default(),
            ui_scale: Default::default(),
        }
    }
}

impl NightgraphApp {

    fn view_settings_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("view_settings_grid")
            .num_columns(2)
            .striped(false)
            .show(ui, |ui| {
                ui.label("Draw debug geometry");
                ui.checkbox(&mut self.drawing.draw_debug_geom, "");
                ui.end_row();
                ui.label("Draw page outline");
                ui.checkbox(&mut self.drawing.draw_page_outline, "");
                ui.end_row();

                ui.label("Page color");
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut self.drawing.bg_color,
                    egui::color_picker::Alpha::OnlyBlend,
                );
                ui.end_row();
            });
    }
}

impl epi::App for NightgraphApp {
    fn name(&self) -> &str {
        "nightgraph ui"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
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
            std::borrow::Cow::Borrowed(include_bytes!("../../../assets/fonts/Jost-400-Book.otf")),
        );
        fonts.font_data.insert(
            "Monofur".to_owned(),
            std::borrow::Cow::Borrowed(include_bytes!("../../../assets/fonts/Monofur_Regular.ttf")),
        );

        // Place prop font at the highest priority for proportional
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Jost*".to_owned());

        // Place prop font at the lowest priority for monospace
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("Jost*".to_owned());

        // Place mono font at the lowest priority for proportional
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .push("Monofur".to_owned());

        // Place mono font at the highest priority for monospace
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "Monofur".to_owned());

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

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        ctx.set_pixels_per_point(1.5);

        egui::SidePanel::left("side_panel")
            //.default_width(240.0)
            .min_width(150.)
            .show(ctx, |ui| {
                ui.heading("nightgraph ui");
                egui::warn_if_debug_build(ui);

                ui.add(egui::Separator::default().spacing(15.));

                ui.collapsing("View Settings", |ui| {
                    self.view_settings_grid(ui);
                });
                ui.collapsing("Sketch Settings", |ui| {
                    self.sketch_control.param_grid(ui);
                });
                if self.sketch_control.needs_render {
                    self.drawing.rerender(self.sketch_control.render().unwrap());
                    self.sketch_control.needs_render = false;
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::dark_canvas(ui.style())
                .fill(self.drawing.bg_color)
                .margin((0., 0.))
                .show(ui, |ui| {
                    self.drawing.ui_content(ui);
                });
        });
    }
}
