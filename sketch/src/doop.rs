use super::*;
use std::f64::consts::{E, PI, TAU};

/// A series of lightly complex sine modulated rings around the center of the
/// page with optional text cutout.
#[sketch]
pub struct Doop {
    /// The number of rings to draw
    #[param(default = 35, range=2..=60)]
    plevels: u64,

    /// The number of steps to sample the sine wave(s) at during a circular
    /// sweep of a ring
    #[param(default = 33, range=2..=50)]
    rotational_steps: u64,

    /// When set, the resulting bloom will be one single path, rotated LEVELS
    /// amount of times, instead of distrete closed paths per LEVEL
    spiral: bool,

    /// Display overlaid text
    display_text: bool,
}

fn exp_dec(lambda: f64, t: f64) -> f64 {
    E.powf(-lambda * t)
}
fn sin_component(amplitude: f64, freq: f64, t: f64, phase: f64) -> f64 {
    amplitude * (TAU * freq * t + phase).sin()
}

impl Sketch for Doop {
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
            .text_line("by")
            .text_line("another")
            .text_line("paralysis")
            .build()
            .unwrap();

        let steps = self.rotational_steps;
        let plevels = self.plevels;
        let th_delta = TAU / steps as f64;
        let mut points = vec![];
        for level in 0..plevels {
            if !self.spiral {
                points.clear()
            };
            let level_prog = level as f64 / plevels as f64;
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
                if self.display_text {
                    let diff = poly.difference(&text);
                    canvas.add(diff);
                } else {
                    canvas.add(poly);
                }
            }
        }
        if self.spiral {
            let path = Path::from_points_smooth(&points);
            if self.display_text {
                let diff = path.difference(&text);
                canvas.add(diff);
            } else {
                canvas.add(path);
            }
        }

        if self.display_text {
            canvas.add(text);
        }

        Ok(canvas)
    }
}
