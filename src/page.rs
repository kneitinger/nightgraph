use crate::geometry::PageSpace;
use crate::geometry::Pathable;
use euclid::Size2D;
use std::fmt;
use svg::node::element::Text;
use svg::node::Text as PrimitiveText;
use svg::Document;

pub enum Unit {
    Px,
    Mm,
    In,
}

pub const DPI: f64 = 96.;
pub const INCH: f64 = DPI;
pub const PX: f64 = 1.;
pub const MM: f64 = DPI / 25.4; // 1in == 25.4mm
pub const CM: f64 = MM * 10.;
pub const PT: f64 = INCH / 72.;

impl Unit {
    pub fn to_string_with_val(&self, n: f64) -> String {
        format!("{}{}", n, self)
    }

    pub fn scale(&self) -> f64 {
        match self {
            Self::Px => 1.,
            Self::Mm => 3.77953,
            Self::In => 96.,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Px => "px",
                Self::Mm => "mm",
                Self::In => "in",
            }
        )
    }
}

pub enum PageType {
    BlackPad,
    A4,
    A6,
    Other(f64, f64, Unit),
}

impl PageType {
    pub fn dimensions(&self) -> Size2D<f64, PageSpace> {
        match self {
            Self::BlackPad => Size2D::new(20.7 * CM, 29.35 * CM),
            Self::A6 => Size2D::new(105. * MM, 148. * MM),
            Self::A4 => Size2D::new(210. * MM, 297. * MM),
            Self::Other(w, h, _) => Size2D::new(w * MM, h * MM),
        }
    }
}

pub struct Page {
    doc: Document,
    //width: f64,
    //height: f64,
    //unit: Unit,
}

impl Page {
    pub fn add<T: Pathable>(&mut self, p: &T) {
        self.doc = self.doc.clone().add(p.to_path());
    }

    pub fn add_comment<T: Into<String>>(&mut self, content: T) {
        let text = Text::new().add(PrimitiveText::new(content));
        self.doc = self.doc.clone().add(text);
    }

    pub fn new(width: f64, height: f64, unit: Unit) -> Page {
        Self {
            doc: Document::new()
                .set("width", unit.to_string_with_val(width))
                .set("height", unit.to_string_with_val(height)),
            //width,
            //height,
            //unit,
        }
    }

    pub fn save<T: AsRef<std::path::Path>>(&self, filepath: T) {
        svg::save(filepath, &self.doc).unwrap()
    }
}
