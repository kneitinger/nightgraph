use crate::geometry::PageSpace;
use crate::geometry::Pathable;
use euclid::Size2D;
use svg::node::element::Text;
use svg::node::Text as PrimitiveText;
use svg::Document;

pub enum PageUnit {
    Px,
    Mm,
    In,
}

const dpi: f64 = 96.;
const inch: f64 = dpi;
const px: f64 = 1.;
const mm: f64 = dpi / 25.4; // 1in == 25.4mm
const cm: f64 = mm * 10.;
const pt: f64 = inch / 72.;

impl PageUnit {
    pub fn to_string(&self) -> String {
        match self {
            PageUnit::Px => "px".to_string(),
            PageUnit::Mm => "mm".to_string(),
            PageUnit::In => "in".to_string(),
        }
    }

    pub fn to_string_with_val(&self, n: f64) -> String {
        format!("{}{}", n, self.to_string())
    }

    pub fn scale(&self) -> f64 {
        match self {
            PageUnit::Px => 1.,
            PageUnit::Mm => 3.77953,
            PageUnit::In => 96.,
        }
    }
}

pub enum PageType {
    BlackPad,
    A4,
    A6,
    Other(f64, f64, PageUnit),
}

impl PageType {
    pub fn dimensions(&self) -> Size2D<f64, PageSpace> {
        match self {
            PageType::BlackPad => Size2D::new(20.7 * cm, 29.35 * cm),
            PageType::A6 => Size2D::new(105. * mm, 148. * mm),
            PageType::A4 => Size2D::new(210. * mm, 297. * mm),
            PageType::Other(w, h, _) => Size2D::new(w * mm, h * mm),
        }
    }
}

pub struct Page {
    doc: Document,
    width: f64,
    height: f64,
    unit: PageUnit,
}

impl Page {
    pub fn add<T: Pathable>(&mut self, p: T) {
        self.doc = self.doc.clone().add(p.to_path());
    }

    pub fn add_comment<T: Into<String>>(&mut self, content: T) {
        let text = Text::new().add(PrimitiveText::new(content));
        self.doc = self.doc.clone().add(text);
    }

    pub fn new(width: f64, height: f64, unit: PageUnit) -> Page {
        Page {
            doc: Document::new()
                .set("width", unit.to_string_with_val(width))
                .set("height", unit.to_string_with_val(height)),
            width,
            height,
            unit,
        }
    }

    pub fn save<T: AsRef<std::path::Path>>(&self, filepath: T) {
        svg::save(filepath, &self.doc).unwrap()
    }
}
