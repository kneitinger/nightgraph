use std::fmt;

pub use kurbo::{Point, Size, Vec2};

#[derive(Copy, Clone)]
pub enum Unit {
    Px,
    Mm,
    In,
    Cm,
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
            Self::Px => PX,
            Self::Mm => MM,
            Self::Cm => CM,
            Self::In => INCH,
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
                Self::Cm => "cm",
                Self::In => "in",
            }
        )
    }
}
