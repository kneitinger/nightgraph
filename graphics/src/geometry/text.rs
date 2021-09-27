use super::{point, GeomError, GeomResult, Path, PathEl, Point, Shaped};
use rusttype::{Font, OutlineBuilder, Scale, Vector};
use std::fs::File;
use std::io::Read;

pub struct TextBuilder<'a> {
    font: Option<&'a str>,
    size: Option<f64>,
    text_lines: Vec<&'a str>,
    line_padding: Option<f64>,
    origin: Option<Point>,
}

#[allow(clippy::new_without_default)]
impl<'a> TextBuilder<'a> {
    pub fn new() -> Self {
        Self {
            font: None,
            size: None,
            text_lines: vec![],
            line_padding: None,
            origin: None,
        }
    }

    pub fn font(mut self, font_path: &'a str) -> Self {
        self.font = Some(font_path);
        self
    }

    pub fn size<T: Into<f64>>(mut self, size: T) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn text_line(mut self, text: &'a str) -> Self {
        self.text_lines.push(text);
        self
    }

    pub fn line_padding(mut self, padding: f64) -> Self {
        self.line_padding = Some(padding);
        self
    }

    pub fn origin(mut self, origin: Point) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn build(self) -> GeomResult<Path> {
        let origin = if let Some(p) = self.origin {
            p
        } else {
            Point::new(0., 0.)
        };
        let size = if let Some(s) = self.size { s } else { 100. } as f32;
        let text_lines = if !self.text_lines.is_empty() {
            self.text_lines
        } else {
            vec!["Lorem Ipsum"]
        };
        let line_padding = if let Some(padding) = self.line_padding {
            padding
        } else {
            50.
        };
        let font_data = if let Some(path) = self.font {
            let buf = &mut [];
            File::open(path)?.read_exact(buf)?;
            buf.to_vec()
        } else {
            include_bytes!("../../assets/Jost-500-Medium.otf").to_vec()
        };

        let font = Font::try_from_vec(font_data).ok_or_else(|| GeomError::font_error(""))?;

        let scale = Scale { x: size, y: size };
        let base_offset = rusttype::point(origin.x as f32, origin.y as f32);
        let mut adj_offset = base_offset;

        let mut paths = vec![];
        let mut combined_cmds = vec![];

        for t in text_lines {
            let mut cmds = vec![];
            let glyphs = font.layout(t, scale, adj_offset);
            for g in glyphs {
                let pos = g.position();

                let mut path_outliner = PathOutlineBuilder::new(point(pos.x, pos.y));
                let unpositioned_glyph = g.unpositioned();
                unpositioned_glyph.build_outline(&mut path_outliner);

                // Whitespace produces a list of empty commands in path_outliner,
                // which when passed to Path::with_commands(cmds) produces an
                // error. In this case, we'll skip any erroneous paths, assuming
                // that whitespace is the only thing that triggers this, however
                // as this is used, if anything else is getting dropped, this
                // approach can be revisited
                if let Ok(path) = path_outliner.path() {
                    for cmd in path.commands() {
                        cmds.push(*cmd);
                        combined_cmds.push(*cmd);
                    }
                }
            }
            let p = Path::with_commands(&cmds)?;
            adj_offset = adj_offset
                + Vector {
                    x: 0.0_f32,
                    y: (p.bounding_box().height() + line_padding) as f32,
                };
            paths.push(p);
        }
        //Ok(paths)
        Path::with_commands(&combined_cmds)
    }
}

struct PathOutlineBuilder {
    cmds: Vec<PathEl>,
    translation: Point,
}

impl PathOutlineBuilder {
    fn new(translation: Point) -> Self {
        Self {
            cmds: vec![],
            translation,
        }
    }

    fn path(&self) -> GeomResult<Path> {
        Path::with_commands(&self.cmds)
    }

    fn point(&self, x: f32, y: f32) -> Point {
        point(x + self.translation.x as f32, y + self.translation.y as f32)
    }
}

impl OutlineBuilder for PathOutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.cmds.push(PathEl::MoveTo(self.point(x, y)));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.cmds.push(PathEl::LineTo(self.point(x, y)));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.cmds
            .push(PathEl::QuadTo(self.point(x1, y1), self.point(x, y)));
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.cmds.push(PathEl::CurveTo(
            self.point(x1, y1),
            self.point(x2, y2),
            self.point(x, y),
        ));
    }
    fn close(&mut self) {
        self.cmds.push(PathEl::ClosePath);
    }
}
