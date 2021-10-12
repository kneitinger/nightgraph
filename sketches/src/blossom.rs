use super::*;
use std::f64::consts::{E, PI, TAU};

/// Sketch description test string
#[derive(Debug, Deserialize, Clone, Serialize, Clap)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, serde::Serialize)
)]
pub struct Blossom {
    #[clap(short, long)]
    text_enabled: bool,

    #[clap(short, long)]
    spiral: bool,
    // it would be cool to have named radio buttons via raw enums,
    // or also to be able to name the true and false vals of bool
    #[clap(short, long, default_value = "10")]
    steps: u64,

    #[clap(short, long, default_value = "65")]
    levels: u64,
    // #[param(range=(1..=2))]
}

impl Default for Blossom {
    fn default() -> Self {
        Self {
            text_enabled: false,
            spiral: false,
            rotational_steps: 75,
            levels: 65,
        }
    }
}

fn exp_dec(lambda: f64, t: f64) -> f64 {
    E.powf(-lambda * t)
}
fn sin_component(amplitude: f64, freq: f64, t: f64, phase: f64) -> f64 {
    amplitude * (TAU * freq * t + phase).sin()
}

impl Sketch for Blossom {
    fn exec(&self) -> SketchResult<Canvas> {
        const WIDTH: f64 = 11. * INCH;
        const HEIGHT: f64 = 17. * INCH;
        let mut canvas = Canvas::new(point(0, 0), Size::new(11. * INCH, 17. * INCH));

        let center = point(WIDTH / 2., HEIGHT / 2.);

        let text = TextBuilder::new()
            .size(300.)
            .origin(point(1. * INCH, 3. * INCH))
            .line_padding(-10.)
            .text_line("a desire")
            .text_line("elided")
            .text_line("by")
            .text_line("another")
            .text_line("paralysis")
            .build()
            .unwrap();

        let steps = self.rotational_steps;
        let levels = self.levels;
        let th_delta = TAU / steps as f64;
        let mut points = vec![];
        for level in 0..levels {
            if !self.spiral {
                points.clear()
            };
            let level_prog = level as f64 / levels as f64;
            for step in 0..steps {
                let th = step as f64 * th_delta;
                let step_prog = step as f64 / steps as f64;
                let base_dist = 4.25 * INCH - 2.5 * level_prog * INCH;
                let sin_mod = sin_component(0.75 * INCH, 5., step_prog, 0. * PI)
                    * exp_dec(4., level_prog)
                    + sin_component(0.25 * INCH, 15., step_prog, PI / 2.) * exp_dec(5., level_prog);
                //+ sin_component(1. * INCH, 2., step_prog, 1.5*PI) * exp_dec(5., level_prog);
                let dist = (base_dist + sin_mod) * Vec2::from_angle(th);
                points.push(center + dist);
            }
            if !self.spiral {
                let poly = Poly::new_smooth(&points);
                if self.text_enabled {
                    let diff = poly.difference(&text);
                    canvas.add(diff);
                } else {
                    canvas.add(poly);
                }
            }
            if self.spiral {
                let path = Path::from_points_smooth(&points);
                if self.text_enabled {
                    let diff = path.difference(&text);
                    canvas.add(diff);
                } else {
                    canvas.add(path);
                }
            }
        }

        if self.text_enabled {
            canvas.add(text);
        }

        Ok(canvas)
    }
}

impl SketchAccess for Blossom {
    fn params(&self) -> Vec<Param> {
        vec![
            Param {
                id: 0,
                name: "levels",
                description: "foo",
                kind: ParamKind::UInt,
                ui_hint: None,
            },
            Param {
                id: 1,
                name: "steps",
                description: "foo",
                kind: ParamKind::UInt,
                ui_hint: None,
            },
            Param {
                id: 2,
                name: "text_enabled",
                description: "foo",
                kind: ParamKind::Bool,
                ui_hint: None,
            },
            Param {
                id: 3,
                name: "spiral",
                description: "foo",
                kind: ParamKind::Bool,
                ui_hint: None,
            },
        ]
    }

    fn mut_float_by_id(&mut self, _id: u8) -> SketchResult<&mut f64> {
        Err(SketchError::ConvertError)
    }

    fn mut_int_by_id(&mut self, _id: u8) -> SketchResult<&mut i64> {
        Err(SketchError::ConvertError)
    }

    fn mut_uint_by_id(&mut self, id: u8) -> SketchResult<&mut u64> {
        match id {
            0 => Ok(&mut self.levels),
            1 => Ok(&mut self.steps),
            _ => Err(SketchError::ConvertError),
        }
    }

    fn mut_bool_by_id(&mut self, id: u8) -> SketchResult<&mut bool> {
        match id {
            2 => Ok(&mut self.text_enabled),
            3 => Ok(&mut self.spiral),
            _ => Err(SketchError::ConvertError),
        }
    }
}
