use eframe::{
    egui,
    egui::{FontDefinitions, FontFamily, Style},
    epi,
};

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

    ui_scale: f32,
}

impl Default for NightgraphApp {
    fn default() -> Self {
        Self {
            drawing: Drawing::default(),
            sketch_control: SketchControl::default(),
            ui_scale: 1.5,
        }
    }
}

impl NightgraphApp {
    fn setup_fonts(&mut self, ctx: &egui::CtxRef) {
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
    }

    pub fn app_settings_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("app_settings_grid")
            .num_columns(2)
            .striped(false)
            .show(ui, |ui| {
                ui.label("Draw debug geometry");
                ui.add(
                    egui::DragValue::new(&mut self.ui_scale)
                        .speed(0.01)
                        .clamp_range(1.0..=2.25),
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

        self.setup_fonts(ctx);

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
        ctx.set_pixels_per_point(self.ui_scale);

        egui::SidePanel::left("side_panel")
            //.default_width(240.0)
            .min_width(150.)
            .show(ctx, |ui| {
                ui.heading("nightgraph ui");
                egui::warn_if_debug_build(ui);

                ui.add(egui::Separator::default().spacing(15.));

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.collapsing("App Settings", |ui| {
                        self.app_settings_grid(ui);
                    });
                    ui.collapsing("Canvas Settings", |ui| {
                        self.drawing.settings_grid(ui);
                    });
                    ui.collapsing("Sketch Settings", |ui| {
                        self.sketch_control.param_grid(ui);
                    });
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
