use eframe::{
    egui,
    egui::{FontDefinitions, FontFamily, Style},
    epi,
};
use nightgraphics::render::EguiRenderer;
use sketches::{Param, ParamKind, SketchList};

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
    #[serde(skip)]
    params: Vec<Param>,
    // TODO: previous sessions mode using persistence.
    // saves the sketch struct values, but not the rendered shapes
}

impl Default for NightgraphApp {
    fn default() -> Self {
        let sketch = SketchList::default();
        let params = sketch.params();
        Self {
            sketch,
            drawing: Drawing::default(),
            params,
        }
    }
}

impl NightgraphApp {
    fn param_grid_contents(&mut self, ui: &mut egui::Ui) {
        for param in &self.params {
            let sketch = &mut self.sketch;
            let drawing = &mut self.drawing;
            let id = param.id();
            match param.kind {
                ParamKind::Text => {
                    // Multiline text box by default, but single line if
                    // a hint exists
                }
                ParamKind::Int => {}
                ParamKind::Float => {}
                ParamKind::UFloat => {}
                ParamKind::UInt => {
                    ui.label(param.name);
                    ui.add(egui::widgets::DragValue::from_get_set(
                        move |v: Option<f64>| {
                            if let Some(v) = v {
                                sketch.set_uint_by_id(id, v as u64).unwrap();
                                println!("sdfsd");
                                drawing.rerender(sketch.exec().unwrap().render_egui())
                            }
                            sketch.get_uint_by_id(id).unwrap() as f64
                        },
                    ));

                    // Number box by default, slider,etc. with hint
                }
                ParamKind::Bool => {
                    // Checkbox/Label Button box by default
                    let val = sketch.get_mut_ref_bool_by_id(id).unwrap();
                    let init_val = *val;

                    ui.label(param.name);
                    ui.add(egui::widgets::Checkbox::new(val, ""));
                    if *val != init_val {
                        drawing.rerender(sketch.exec().unwrap().render_egui());
                    }
                }
            }
            ui.end_row();
        }
    }
    fn param_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("params_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(false)
            .show(ui, |ui| self.param_grid_contents(ui));
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

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        /*
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
        */

        egui::SidePanel::left("side_panel")
            //.default_width(240.0)
            .min_width(150.)
            .show(ctx, |ui| {
                ui.heading("nightgraph ui");
                egui::warn_if_debug_build(ui);

                ui.add(egui::Separator::default().spacing(15.));

                ui.collapsing("View Settings", |ui| {
                    ui.checkbox(&mut self.drawing.draw_debug_geom, "Draw debug geometry");
                    ui.checkbox(&mut self.drawing.draw_page_outline, "Draw page outline");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut self.drawing.bg_color,
                        egui::color_picker::Alpha::OnlyBlend,
                    );
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
