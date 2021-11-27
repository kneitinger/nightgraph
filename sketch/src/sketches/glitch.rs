use super::*;

/// A heavily-differenced set of words and circles designed to embrace
/// the glitchy (at time of creation) edge cases of the difference function
#[sketch]
pub struct Glitch {
    #[param(default = 1.9)]
    font_size: f64,

    #[param(default = 0.8)]
    font_y_offset: f64,

    #[param(default = 0.45)]
    font_line_padding: f64,

    #[param(default = 80)]
    horizontal_lines: u64,

    #[param(default = 0.5)]
    margin: f64,
}

impl Sketch for Glitch {
    fn exec(&self) -> SketchResult<Canvas> {
        const WIDTH: f64 = 11. * INCH;
        const HEIGHT: f64 = 17. * INCH;
        let mut canvas = Canvas::new(point(0, 0), Size::new(WIDTH, HEIGHT));
        let center = point(WIDTH / 2., HEIGHT / 2.);
        let margin = self.margin * INCH;

        let raw_text = include_str!("../../../assets/data/glitch_sketch_text.txt");

        let text_lines: Vec<&str> = raw_text.lines().map(|s| s.trim().trim_start()).collect();

        let base_font_origin = point(1. * INCH, 1. * INCH + self.font_y_offset * INCH);
        let text_paths: Vec<Path> = text_lines
            .iter()
            .enumerate()
            .map(|(n, line)| {
                let text_origin =
                    base_font_origin + Vec2::new(0., n as f64 * self.font_line_padding * INCH);
                TextBuilder::new()
                    .text_line(line)
                    .origin(text_origin)
                    .size(self.font_size * INCH)
                    .stroke_width(0.7 * MM)
                    .build()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let line_spacing = (HEIGHT - 2. * margin) / self.horizontal_lines as f64;
        let horizontal_lines: Vec<Path> = (0..self.horizontal_lines)
            .map(|n| Circle::new(center, n as f64 * line_spacing).to_path())
            .collect();

        let mut res_paths: Vec<Path> = vec![];
        for p in text_paths.iter().step_by(2) {
            let mut diffed_path = p.clone();
            for r in &res_paths {
                diffed_path = diffed_path.difference(r);
            }
            diffed_path.stroke_width = 0.7 * MM;
            res_paths.push(diffed_path);
        }

        for p in text_paths.iter().skip(1).step_by(2) {
            let mut diffed_path = p.clone();
            for r in &res_paths {
                diffed_path = diffed_path.difference(r);
            }
            diffed_path.stroke_width = 0.7 * MM;
            res_paths.push(diffed_path);
        }

        for l in horizontal_lines {
            let mut diffed_path = l.clone();
            for r in &res_paths {
                diffed_path = diffed_path.difference(r);
            }
            res_paths.push(diffed_path);
        }

        for p in res_paths {
            canvas.add(p);
        }
        Ok(canvas)
    }
}
