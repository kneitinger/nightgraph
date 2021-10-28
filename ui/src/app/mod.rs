use eframe::{
    egui,
    egui::{FontDefinitions, FontFamily, Style},
    epi,
};
use nightgraphics::render::EguiRenderer;
use nightsketch::{ParamKind, ParamMetadata, ParamRange, SketchList};

mod drawing;
use drawing::Drawing;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct NightgraphApp {
    // Temporarily opt out of state persistence on drawing until the sketch
    // and associated info is actually stored in the app state
    #[serde(skip)]
    drawing: Drawing,

    sketch: SketchList,

    ui_scale: Option<f32>,

    // TODO: previous sessions mode using persistence.
    // saves the sketch struct values, but not the rendered shapes
    #[serde(skip)]
    params: Vec<ParamMetadata>,
}

impl Default for NightgraphApp {
    fn default() -> Self {
        let sketch = SketchList::default();
        let params = sketch.param_metadata();
        Self {
            sketch,
            drawing: Drawing::default(),
            params,
            ui_scale: Default::default(),
        }
    }
}

impl NightgraphApp {
    fn param_grid_contents(&mut self, ui: &mut egui::Ui) {
        for param in &self.params {
            let sketch = &mut self.sketch;
            let drawing = &mut self.drawing;
            let id = param.id;
            match param.kind {
                ParamKind::Int => {
                    ui.label(param.name);
                    let val = sketch.mut_int_by_id(id).unwrap();
                    let init = *val;
                    let dragval = if let Some(ParamRange::Int(range)) = &param.range {
                        egui::widgets::DragValue::new(val).clamp_range(range.to_owned())
                    } else {
                        egui::widgets::DragValue::new(val)
                    };
                    ui.add(dragval);
                    if *val != init {
                        drawing.rerender(sketch.exec().unwrap().render_egui());
                    }
                }
                ParamKind::Float => {
                    ui.label(param.name);
                    let val = sketch.mut_float_by_id(id).unwrap();
                    let init = *val;
                    let dragval = if let Some(ParamRange::Float(range)) = &param.range {
                        egui::widgets::DragValue::new(val).clamp_range(range.to_owned())
                    } else {
                        egui::widgets::DragValue::new(val)
                    };
                    ui.add(dragval);
                    if (*val - init).abs() > f64::EPSILON {
                        drawing.rerender(sketch.exec().unwrap().render_egui());
                    }
                }
                ParamKind::UInt => {
                    ui.label(param.name);
                    let val = sketch.mut_uint_by_id(id).unwrap();
                    let init = *val;
                    let dragval = if let Some(ParamRange::Int(range)) = &param.range {
                        egui::widgets::DragValue::new(val).clamp_range(range.to_owned())
                    } else {
                        egui::widgets::DragValue::new(val)
                    };
                    ui.add(dragval);
                    if *val != init {
                        drawing.rerender(sketch.exec().unwrap().render_egui());
                    }
                }
                ParamKind::Bool => {
                    // Checkbox/Label Button box by default
                    let val = sketch.mut_bool_by_id(id).unwrap();
                    let init = *val;

                    ui.label(param.name);
                    ui.add(egui::widgets::Checkbox::new(val, ""));
                    if *val != init {
                        drawing.rerender(sketch.exec().unwrap().render_egui());
                    }
                }
                // TODO: Showing a label with param name and unsupported would by nice
                ParamKind::Unsupported => {}
            }
            ui.end_row();
        }
    }
    fn param_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("params_grid")
            .num_columns(2)
            .spacing([55.0, 4.0])
            .striped(false)
            .show(ui, |ui| self.param_grid_contents(ui));
    }

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
                self.param_grid(ui);
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
