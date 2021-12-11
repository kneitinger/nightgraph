use eframe::egui;
use nightgraphics::render::EguiRenderer;
use nightsketch::*;

pub struct SketchControl {
    sketch: Box<dyn Sketch>,
    sketch_name: String,
    params: Vec<ParamMetadata>,
    sketch_names: Vec<String>,
    pub needs_render: bool,
}

impl Default for SketchControl {
    fn default() -> Self {
        let sketch = SketchList::default_sketch();
        let sketch_name = "Blossom".to_string();
        let params = sketch.param_metadata();
        let sketch_names = SketchList::sketch_names();
        SketchControl {
            sketch,
            sketch_name,
            params,
            sketch_names,
            needs_render: true,
        }
    }
}

impl SketchControl {
    fn param_grid_contents(&mut self, ui: &mut egui::Ui) {
        ui.label("Sketch");
        let val = self.sketch_name.to_string();
        egui::ComboBox::from_label("")
            .selected_text(self.sketch_name.to_string())
            .show_ui(ui, |ui| {
                for n in &self.sketch_names {
                    ui.selectable_value(&mut self.sketch_name, n.to_string(), n);
                }
            });
        if val != self.sketch_name {
            self.sketch = SketchList::sketch_by_name(&self.sketch_name).unwrap();
            self.needs_render = true;
            self.params = self.sketch.param_metadata();
        }
        ui.end_row();
        // Leave some visual space without a separator
        ui.end_row();

        for param in &self.params {
            let sketch = &mut self.sketch;
            let needs_render = &mut self.needs_render;
            let id = param.id;
            match param.kind {
                ParamKind::Int => {
                    ui.label(param.name);
                    ui.horizontal(|ui| {
                        let val = sketch.mut_int_by_id(id).unwrap();
                        let init = *val;
                        let dragval = if let Some(ParamRange::Int(range)) = &param.range {
                            egui::widgets::DragValue::new(val).clamp_range(range.to_owned())
                        } else {
                            egui::widgets::DragValue::new(val)
                        };
                        ui.add(dragval);
                        if ui.button("↺").clicked() {
                            *val = match param.default {
                                ParamDefault::Int(i) => i,
                                _ => Default::default(),
                            }
                        }
                        if *val != init {
                            *needs_render = true;
                        }
                    });
                }
                ParamKind::Float => {
                    ui.label(param.name);
                    ui.horizontal(|ui| {
                        let val = sketch.mut_float_by_id(id).unwrap();
                        let init = *val;
                        let dragval = if let Some(ParamRange::Float(range)) = &param.range {
                            egui::widgets::DragValue::new(val).clamp_range(range.to_owned())
                        } else {
                            egui::widgets::DragValue::new(val)
                        };
                        ui.add(dragval);
                        if ui.button("↺").clicked() {
                            *val = match param.default {
                                ParamDefault::Float(f) => f,
                                _ => Default::default(),
                            }
                        }
                        if (*val - init).abs() > f64::EPSILON {
                            *needs_render = true;
                        }
                    });
                }
                ParamKind::UInt => {
                    ui.label(param.name);
                    ui.horizontal(|ui| {
                        let val = sketch.mut_uint_by_id(id).unwrap();
                        let init = *val;
                        let dragval = if let Some(ParamRange::Int(range)) = &param.range {
                            egui::widgets::DragValue::new(val).clamp_range(range.to_owned())
                        } else {
                            egui::widgets::DragValue::new(val)
                        };
                        ui.add(dragval);
                        if ui.button("↺").clicked() {
                            *val = match param.default {
                                ParamDefault::UInt(i) => i,
                                _ => Default::default(),
                            }
                        }
                        if *val != init {
                            *needs_render = true;
                        }
                    });
                }
                ParamKind::Bool => {
                    // Checkbox/Label Button box by default
                    let val = sketch.mut_bool_by_id(id).unwrap();
                    let init = *val;

                    ui.label(param.name);
                    ui.horizontal(|ui| {
                        ui.add(egui::widgets::Checkbox::new(val, ""));
                        if ui.button("↺").clicked() {
                            *val = match param.default {
                                ParamDefault::Bool(b) => b,
                                _ => Default::default(),
                            }
                        }
                        if *val != init {
                            *needs_render = true;
                        }
                    });
                }
                // TODO: Showing a label with param name and unsupported would by nice
                ParamKind::Unsupported => {}
            }
            ui.end_row();
        }
    }
    pub fn param_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("params_grid")
            .num_columns(2)
            .spacing([20.0, 4.0])
            .striped(false)
            .show(ui, |ui| self.param_grid_contents(ui));
    }

    pub fn render(&self) -> SketchResult<(egui::Vec2, Vec<egui::Shape>)> {
        Ok(self.sketch.exec()?.render_egui())
    }
}
