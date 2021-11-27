use super::*;

/// A sweeping curtain of paths that flow from tightly packed to loosely
/// spaced, with optional text
#[sketch]
pub struct Postcard {
    #[param(default = 139.0)]
    text_x: f64,
    #[param(default = 117.0)]
    text_y: f64,
    #[param(default = 90.0)]
    text_size: f64,
    #[param(default = -10.0)]
    text_padding: f64,

    #[param(default = 68.0)]
    wave_a_base: f64,
    #[param(default = 231.0)]
    wave_b_base: f64,

    #[param(default = 0.3)]
    wave_a0_amp: f64,
    #[param(default = 2.1)]
    wave_a0_freq: f64,
    #[param(default = 0.)]
    wave_a0_phase: f64,
    #[param(default = 0.3)]
    wave_a1_amp: f64,
    #[param(default = 3.7)]
    wave_a1_freq: f64,
    #[param(default = 1.1)]
    wave_a1_phase: f64,

    #[param(default = 1.)]
    wave_b0_amp: f64,
    #[param(default = -0.9)]
    wave_b0_freq: f64,
    #[param(default = 0.)]
    wave_b0_phase: f64,
    #[param(default = 1.)]
    wave_b1_amp: f64,
    #[param(default = -0.6)]
    wave_b1_freq: f64,
    #[param(default = -9.1)]
    wave_b1_phase: f64,

    #[param(default = 67, range = 10..=200)]
    sine_samples: u64,

    #[param(default = 52, range = 1..=200)]
    sine_waves: u64,

    #[param(default = 0.25)]
    margin: f64,
}

impl Postcard {
    fn wave_a(&self, t: f64) -> f64 {
        sine_wave(
            self.wave_a0_amp * INCH,
            self.wave_a0_freq,
            t,
            self.wave_a0_phase,
        ) + sine_wave(
            self.wave_a1_amp * INCH,
            self.wave_a1_freq,
            t,
            self.wave_a1_phase,
        )
    }

    fn wave_b(&self, t: f64) -> f64 {
        sine_wave(
            self.wave_b0_amp * INCH,
            self.wave_b0_freq,
            t,
            self.wave_b0_phase,
        ) + sine_wave(
            self.wave_b1_amp * INCH,
            self.wave_b1_freq,
            t,
            self.wave_b1_phase,
        )
    }
}

impl Sketch for Postcard {
    fn exec(&self) -> SketchResult<Canvas> {
        const WIDTH: f64 = 6. * INCH;
        const HEIGHT: f64 = 5. * INCH;
        let mut canvas = Canvas::new(point(0, 0), Size::new(WIDTH, HEIGHT));
        let margin = self.margin * INCH;
        let width_adj = WIDTH - 2. * margin;

        let mut waves: Vec<Vec<Point>> = vec![vec![]; self.sine_waves as usize];

        let x_interval = width_adj / self.sine_samples as f64;
        for n in 0..self.sine_samples {
            let t_x = n as f64 / self.sine_samples as f64;
            let samp_a = self.wave_a(t_x);
            let samp_b = self.wave_b(t_x);

            let x = margin + x_interval * n as f64;

            for l in 0..self.sine_waves {
                let t_y = l as f64 / self.sine_waves as f64;
                let point_a = point(x, self.wave_a_base + samp_a);
                let point_b = point(x, self.wave_b_base + samp_b);

                let lerped = point_a.lerp(point_b, t_y);
                waves[l as usize].push(lerped);
            }
        }

        let text = TextBuilder::new()
            .text_line("proto")
            .text_line("permanence")
            .origin(point(self.text_x, self.text_y))
            .size(self.text_size)
            .line_padding(self.text_padding)
            .build()?;

        for wave in waves {
            let p = PathBuilder::new().points(&wave).smooth().build()?;
            let d = p.difference(&text);
            canvas.add(d);
        }
        canvas.add(text);

        Ok(canvas)
    }
}
