use super::*;
use std::f64::consts::PI;

/// A rendition of a droplet of water hitting a surface and waves
/// emanating outward
#[sketch]
pub struct Weather {
    #[param(default = 0.37)]
    wave_amp: f64,

    #[param(default = 4.3)]
    text_y_offset: f64,

    #[param(default = 4.8, range = -4.0..=8.0)]
    decay: f64,

    #[param(default = 75, range = 10..=300)]
    steps: u64,

    #[param(default = 6.4, range = 0.25..=8.0)]
    freq: f64,

    #[param(default = 0.57, range = 0.25..=4.0)]
    xy_ratio: f64,

    #[param(default = 10)]
    drop_steps: u64,

    #[param(default = 89.6)]
    drop_height: f64,

    #[param(default = -4.0, range = -4.0..=4.0)]
    center_y_offset: f64,
}

impl Weather {
    fn drop(&self, origin: Point) -> Vec<Ellipse> {
        let mut ellipses = vec![];
        for n in 0..self.drop_steps {
            let radius_scale = sine_wave(0.05 * INCH, 0.5, n as f64 / self.drop_steps as f64, 0.);
            let x_radius = 0.025 * INCH * radius_scale;
            let y_radius = x_radius * self.xy_ratio;
            let e = Ellipse::new(
                origin + Vec2::new(0., -0.03 * INCH * n as f64),
                (x_radius, y_radius),
                0.,
            );
            ellipses.push(e);
        }
        ellipses
    }
    fn ripple(&self, center: Point, freq: f64) -> Vec<Ellipse> {
        let mut ellipses = vec![];
        for n in 0..self.steps {
            let height_mod = sine_wave(
                self.wave_amp * INCH,
                freq,
                n as f64 / self.steps as f64,
                -PI / 2.,
            ) * exp_dec(self.decay, n as f64 / self.steps as f64);
            //+ sine_wave(2.0 * self.wave_amp * INCH, 0.25, n as f64 / 50., PI / 2.);

            //let height_mod = 0.05 * INCH * n as f64;
            let x_rad_mod = 0.06 * INCH * n as f64;
            let y_rad_mod = 0.06 * self.xy_ratio * INCH * n as f64;
            ellipses.push(Ellipse::new(
                point(center.x, center.y + height_mod),
                (
                    0.125 * INCH + x_rad_mod,
                    0.125 * self.xy_ratio * INCH + y_rad_mod,
                ),
                0.,
            ));
        }
        ellipses
    }
}

impl Sketch for Weather {
    fn exec(&self) -> SketchResult<Canvas> {
        const WIDTH: f64 = 11. * INCH;
        const HEIGHT: f64 = 17. * INCH;

        let mut canvas = Canvas::new(point(0, 0), Size::new(WIDTH, HEIGHT));
        let center = point(WIDTH / 2., HEIGHT / 2. - self.center_y_offset * INCH);

        let text_lines: Vec<&'static str> = vec![
            //"a drop on the",
            "a surface",
            "once untouched",
            "  by our storm.",
            "longing for",
            "  the art of",
            "    interference",
        ];
        let line_count = text_lines.len();

        let mut text_paths: Vec<Path> = vec![];
        for (n, text_line) in text_lines.iter().enumerate().take(line_count) {
            let t = TextBuilder::new()
                .origin(point(
                    0.75 * INCH,
                    (0.5 + self.text_y_offset) * INCH + 1.25 * INCH * n as f64,
                ))
                .size(1.75 * INCH)
                .text_line(text_line)
                .build()?;

            text_paths.push(t);
        }

        for n in 1..line_count {
            let p = text_paths[n - 1].difference(&text_paths[n]);
            canvas.add(p);
        }
        canvas.add(text_paths[line_count - 1].clone());

        let ellipse_rings = self.ripple(center, self.freq);
        for e in ellipse_rings {
            let mut p = e.to_path();
            for n in 0..line_count {
                p = p.difference(&text_paths[n]);
            }
            canvas.add(p);
        }

        let d = self.drop(center + Vec2::new(0., -self.drop_height));
        for e in d {
            canvas.add(e);
        }

        Ok(canvas)
    }
}
