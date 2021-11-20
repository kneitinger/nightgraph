use super::*;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::f64::consts::{PI, TAU};

/// A set of concentric chaotic semi-circles designed to be scratched in
/// to a paper covered with charcoal powder
#[sketch]
pub struct Charcoal {
    #[param(default = 94.0)]
    text_x: f64,
    #[param(default = 117.0)]
    text_y: f64,
    #[param(default = 152.0)]
    text_size: f64,
    #[param(default = -10.0)]
    text_padding: f64,

    #[param(default = 0.25)]
    margin: f64,

    #[param(default = 12)]
    line_count: u64,

    #[param(default = 65.)]
    line_spacing: f64,

    #[param(default = 912.)]
    circle_origin_x: f64,
    #[param(default = 739.)]
    circle_origin_y: f64,

    #[param(default = 0.)]
    circle_base_radius: f64,

    #[param(default = 85, range= 2..=1000)]
    semi_circle_segs: u64,

    #[param(default = 434)]
    seed: u64,

    #[param(default = 0.7, range = 0.01..=3.0)]
    rand_limit: f64,
}

impl Charcoal {}

impl Sketch for Charcoal {
    fn exec(&self) -> SketchResult<Canvas> {
        const WIDTH: f64 = 11. * INCH;
        const HEIGHT: f64 = 17. * INCH;
        let mut canvas = Canvas::new(point(0, 0), Size::new(WIDTH, HEIGHT));
        let margin = self.margin * INCH;
        let _width_adj = WIDTH - 2. * margin;

        let mut rng = Pcg64::seed_from_u64(self.seed);
        let circ_origin = point(self.circle_origin_x, self.circle_origin_y);

        for n in 0..self.line_count {
            let seg_delta = (TAU / 4.) / self.semi_circle_segs as f64;
            let mut circ_arc_points = vec![];

            let radius = (self.circle_base_radius + self.line_spacing) * (n + 1) as f64;

            for j in 0..self.semi_circle_segs {
                let t = rng.gen_range(0. ..self.rand_limit) + (0.5 * PI) + seg_delta * j as f64;

                let x_val = t.cos();
                let y_val = t.sin();
                let point = circ_origin + Vec2::new(x_val * radius, y_val * radius);
                circ_arc_points.push(point);
            }
            let p = PathBuilder::new().points(&circ_arc_points).build()?;
            canvas.add(p);
        }

        let text = TextBuilder::new()
            .origin(point(self.text_x, self.text_y))
            .line_padding(self.text_padding)
            .size(self.text_size)
            .text_line("DEFLECTOR.")
            .font("/usr/share/fonts/TTF/DejaVuSerifCondensed-BoldItalic.ttf")
            .build()?;

        canvas.add(text);

        Ok(canvas)
    }
}
