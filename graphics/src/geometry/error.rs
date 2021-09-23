use std::error::Error;
use std::fmt;

pub type GeomResult<T> = Result<T, GeomError>;

#[derive(Debug)]
pub enum GeomError {
    PathError(String),
    MalformedPath(String),
    MalformedPoly(String),
    FontError(String),
    IoError(std::io::Error),
}
impl From<std::io::Error> for GeomError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl GeomError {
    pub fn path_error(msg: &str) -> Self {
        Self::PathError(msg.to_string())
    }
    #[allow(dead_code)]
    pub fn malformed_poly(msg: &str) -> Self {
        Self::MalformedPoly(msg.to_string())
    }
    pub fn malformed_path(msg: &str) -> Self {
        Self::MalformedPath(msg.to_string())
    }
    pub fn font_error(msg: &str) -> Self {
        Self::FontError(msg.to_string())
    }
}

impl fmt::Display for GeomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PathError(msg) => write!(f, "PathError: {}", msg),
            Self::MalformedPoly(msg) => write!(f, "MalformedPoly: {}", msg),
            Self::MalformedPath(msg) => write!(f, "MalformedPath: {}", msg),
            Self::FontError(msg) => write!(f, "FontError: {}", msg),
            Self::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl Error for GeomError {}
