use crate::geometry::GeomError;
use std::error;
use std::fmt;

pub mod egui;
pub mod svg;

pub type RenderResult<T> = Result<T, RenderError>;

#[derive(Debug)]
pub enum RenderError {
    BaseGeometry(GeomError),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BaseGeometry(e) => write!(f, "error with base geometry: {}", e),
        }
    }
}

impl From<GeomError> for RenderError {
    fn from(err: GeomError) -> Self {
        Self::BaseGeometry(err)
    }
}

impl error::Error for RenderError {}
