use super::*;
use std::f64::consts::{E, PI, TAU};

/// Sketch description test string
#[derive(Default, Debug, Deserialize, Clone, Serialize, Clap)]
pub struct Blossom {
    #[clap(short, long)]
    text_enabled: bool,

    #[clap(short, long, default_value = "10")]
    steps: u64,

    #[clap(short, long, default_value = "65")]
    levels: u64,
}

fn exp_dec(lambda: f64, t: f64) -> f64 {
    E.powf(-lambda * t)
}
fn sin_component(amplitude: f64, freq: f64, t: f64, phase: f64) -> f64 {
    amplitude * (TAU * freq * t + phase).sin()
}
impl SketchExec for Blossom {
    fn exec(&self) -> SketchResult<Canvas> {
        const WIDTH: f64 = 11. * INCH;
        const HEIGHT: f64 = 17. * INCH;
        let mut canvas = Canvas::new(point(0, 0), Size::new(11. * INCH, 17. * INCH));

        let center = point(WIDTH / 2., HEIGHT / 2.);

        let text = TextBuilder::new()
            .size(300.)
            .font("/usr/share/fonts/jost/Jost-400-Book.otf")
            .origin(point(1. * INCH, 3. * INCH))
            .line_padding(-10.)
            .text_line("a desire")
            .text_line("elided")
            .text_line("by")
            .text_line("another")
            .text_line("paralysis")
            .build()
            .unwrap();

        let steps = 75;
        let levels = 65;
        let th_delta = TAU / steps as f64;
        let mut points = vec![];
        for level in 0..levels {
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
        }
        let curvy_path = Path::from_points_smooth(&points);
        if self.text_enabled {
            let diff = curvy_path.difference(&text);
            canvas.add(diff);
        } else {
            canvas.add(curvy_path);
        }

        if self.text_enabled {
            canvas.add(text);
        }

        Ok(canvas)
    }
}
