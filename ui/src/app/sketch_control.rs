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
                        self.needs_render = true;
                        //drawing.rerender(sketch.exec().unwrap().render_egui());
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
                        self.needs_render = true;
                        //drawing.rerender(sketch.exec().unwrap().render_egui());
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
                        self.needs_render = true;
                        //drawing.rerender(sketch.exec().unwrap().render_egui());
                    }
                }
                ParamKind::Bool => {
                    // Checkbox/Label Button box by default
                    let val = sketch.mut_bool_by_id(id).unwrap();
                    let init = *val;

                    ui.label(param.name);
                    ui.add(egui::widgets::Checkbox::new(val, ""));
                    if *val != init {
                        self.needs_render = true;
                        //drawing.rerender(sketch.exec().unwrap().render_egui());
                    }
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
            .spacing([55.0, 4.0])
            .striped(false)
            .show(ui, |ui| self.param_grid_contents(ui));
    }

    pub fn render(&self) -> SketchResult<(egui::Vec2, Vec<egui::Shape>)> {
        Ok(self.sketch.exec()?.render_egui())
    }
}
