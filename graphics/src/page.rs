use crate::geometry::{PageSpace, Pathable};
use crate::units::*;
use euclid::Size2D;
use svg::{
    node::{
        element::{Group as PrimitiveGroup, Text},
        Node, Text as PrimitiveText,
    },
    Document,
};

pub enum PageType {
    BlackPad,
    A4,
    A6,
    Pad11x14,
    Envelope10,
    Other(f64, f64, Unit),
}

impl PageType {
    pub fn dimensions(&self) -> Size2D<f64, PageSpace> {
        match self {
            Self::BlackPad => Size2D::new(20.7, 29.35),  // cm
            Self::A6 => Size2D::new(105., 148.),         // mm
            Self::A4 => Size2D::new(210., 297.),         // mm
            Self::Pad11x14 => Size2D::new(11., 14.),     // in
            Self::Envelope10 => Size2D::new(9.5, 4.125), // in
            Self::Other(w, h, _) => Size2D::new(*w, *h), // other
        }
    }

    pub fn unit(&self) -> Unit {
        match self {
            Self::BlackPad => Unit::Cm,
            Self::A6 => Unit::Cm,
            Self::A4 => Unit::Cm,
            Self::Pad11x14 => Unit::In,
            Self::Envelope10 => Unit::In,
            Self::Other(_, _, u) => *u,
        }
    }
}

pub struct Group {
    raw_group: PrimitiveGroup,
}

// Todo: change pathable to mean addable, so that groups can be added the same way
impl Group {
    pub fn new(name: &str) -> Self {
        let raw_group = PrimitiveGroup::new()
            .set("groupmode", "layer")
            .set("label", name);
        Group { raw_group }
    }

    pub fn add<U: Node, T: Pathable<U>>(&mut self, p: &T) {
        self.raw_group.append(p.to_path());
    }

    fn get_raw(&self) -> PrimitiveGroup {
        self.raw_group.clone()
    }
}

pub struct Page {
    doc: Document,
    width: f64,
    height: f64,
    unit: Unit,
}

impl Page {
    pub fn new(width: f64, height: f64, unit: Unit) -> Page {
        Self {
            doc: Document::new()
                .set("width", unit.to_string_with_val(width))
                .set("height", unit.to_string_with_val(height)),
            width: width * unit.scale(),
            height: height * unit.scale(),
            unit,
        }
    }

    pub fn new_from_pagetype(pagetype: PageType) -> Page {
        let dimensions = pagetype.dimensions();
        Page::new(dimensions.width, dimensions.height, pagetype.unit())
    }

    pub fn add<U: Node, T: Pathable<U>>(&mut self, p: &T) {
        self.doc.append(p.to_path());
    }

    pub fn add_comment<T: Into<String>>(&mut self, content: T) {
        let text = Text::new().add(PrimitiveText::new(content));
        self.doc.append(text);
    }

    pub fn add_group(&mut self, group: &Group) {
        self.doc.append(group.get_raw());
    }

    pub fn save<T: AsRef<std::path::Path>>(&self, filepath: T) {
        svg::save(filepath, &self.doc).expect("Unable to save SVG");
    }

    pub fn write(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        svg::write(&mut vec, &self.doc).expect("Unable to write SVG to bytestream");
        vec
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn dimensions(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    pub fn unit(&self) -> Unit {
        self.unit
    }
}
