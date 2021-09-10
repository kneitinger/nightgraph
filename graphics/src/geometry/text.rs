use crate::geometry::{point, Path, Pathable, Point};
use rusttype::{Font, OutlineBuilder, Scale};

pub struct Text<'a> {
    font: Font<'a>,
    size: f32,
    text: String,
    origin: Point,
}

impl Default for Text<'_> {
    fn default() -> Self {
        let font_data = include_bytes!("../../assets/Jost-500-Medium.otf");
        Self {
            font: Font::try_from_bytes(font_data as &[u8])
                .expect("error constructing a Font from bytes"),
            size: 24.0,
            text: "Lorem Ipsum".to_string(),
            origin: point(0, 0),
        }
    }
}

impl Text<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_font<'a>(&'a mut self, font_path: &str) -> &'a mut Self {
        let data = std::fs::read(&font_path).unwrap();
        self.font = Font::try_from_vec(data).unwrap_or_else(|| {
            panic!("error constructing a Font from data at {:?}", font_path);
        });
        self
    }
    pub fn set_size(&mut self, size: f32) -> &mut Self {
        self.size = size;
        self
    }
    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self
    }
    pub fn set_origin(&mut self, origin: Point) -> &mut Self {
        self.origin = origin;
        self
    }
}

impl Pathable for Text<'_> {
    fn to_path(&self) -> Path {
        let scale = Scale {
            x: self.size,
            y: self.size,
        };
        let offset = rusttype::point(self.origin.x as f32, self.origin.y as f32);

        let mut path = Path::new();

        let glyphs = self.font.layout(&self.text, scale, offset);
        for g in glyphs {
            let pos = g.position();

            let mut path_outliner = PathOutlineBuilder::new(point(pos.x, pos.y));
            let unpositioned_glyph = g.unpositioned();
            unpositioned_glyph.build_outline(&mut path_outliner);
            path.append(path_outliner.path());
        }
        path
    }
}

struct PathOutlineBuilder {
    path: Path,
    translation: Point,
}

impl PathOutlineBuilder {
    fn new(translation: Point) -> Self {
        Self {
            path: Path::new(),
            translation,
        }
    }

    fn path(&self) -> Path {
        self.path.clone()
    }

    fn point(&self, x: f32, y: f32) -> Point {
        point(x + self.translation.x as f32, y + self.translation.y as f32)
    }
}

impl OutlineBuilder for PathOutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path.move_to(self.point(x, y));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to(self.point(x, y));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path.quad_to(self.point(x1, y1), self.point(x, y));
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path
            .curve_to(self.point(x1, y1), self.point(x2, y2), self.point(x, y));
    }
    fn close(&mut self) {
        self.path.close();
    }
}
