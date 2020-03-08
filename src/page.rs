use crate::geometry::{PageSpace, Pathable};
use crate::units::*;
use euclid::Size2D;
use svg::node::element::Text;
use svg::node::Node;
use svg::node::Text as PrimitiveText;
use svg::Document;

pub enum PageType {
    BlackPad,
    A4,
    A6,
    Pad11x14,
    Other(f64, f64, Unit),
}

impl PageType {
    pub fn dimensions(&self) -> Size2D<f64, PageSpace> {
        match self {
            Self::BlackPad => Size2D::new(20.7, 29.35),  // cm
            Self::A6 => Size2D::new(105., 148.),         // mm
            Self::A4 => Size2D::new(210., 297.),         // mm
            Self::Pad11x14 => Size2D::new(11., 14.),     // in
            Self::Other(w, h, _) => Size2D::new(*w, *h), // other
        }
    }

    pub fn unit(&self) -> Unit {
        match self {
            Self::BlackPad => Unit::Cm,
            Self::A6 => Unit::Cm,
            Self::A4 => Unit::Cm,
            Self::Pad11x14 => Unit::In,
            Self::Other(_, _, u) => *u,
        }
    }
}

pub struct Page {
    doc: Document,
    /* For future use
    _width: f64,
    _height: f64,
    _unit: Unit,
    */
}

impl Page {
    pub fn add<T: Pathable>(&mut self, p: &T) {
        self.doc.append(p.to_path());
    }

    pub fn add_comment<T: Into<String>>(&mut self, content: T) {
        let text = Text::new().add(PrimitiveText::new(content));
        self.doc.append(text);
    }

    pub fn new(width: f64, height: f64, unit: Unit) -> Page {
        Self {
            doc: Document::new()
                .set("width", unit.to_string_with_val(width))
                .set("height", unit.to_string_with_val(height)),
        }
    }

    pub fn new_from_pagetype(pagetype: PageType) -> Page {
        let dimensions = pagetype.dimensions();
        Page::new(dimensions.width, dimensions.height, pagetype.unit())
    }

    pub fn save<T: AsRef<std::path::Path>>(&self, filepath: T) {
        svg::save(filepath, &self.doc).expect("sdfds");
    }
}
