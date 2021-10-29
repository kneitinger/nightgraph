use crate::metadata::*;
use nightgraphics::prelude::{Canvas, GeomError};

pub trait Sketch: SketchAccess {
    fn exec(&self) -> SketchResult<Canvas>;
}

pub trait SketchAccess {
    fn param_metadata(&self) -> Vec<ParamMetadata>;
    fn mut_float_by_id(&mut self, id: u64) -> SketchResult<&mut f64>;
    fn mut_int_by_id(&mut self, id: u64) -> SketchResult<&mut i64>;
    fn mut_uint_by_id(&mut self, id: u64) -> SketchResult<&mut u64>;
    fn mut_bool_by_id(&mut self, id: u64) -> SketchResult<&mut bool>;

    fn get_kind_by_id(&mut self, id: u64) -> SketchResult<ParamKind> {
        Ok(self
            .param_metadata()
            .iter()
            .find(|p| p.id == id)
            .ok_or_else(|| SketchError::ParamError(format!("Invalid id: {}", id)))?
            .kind)
    }
}

pub type SketchResult<T> = Result<T, SketchError>;

#[derive(Debug)]
pub enum SketchError {
    Todo(String),
    ParamError(String),
    GraphicsError(GeomError),
    ConvertError,
}

impl From<GeomError> for SketchError {
    fn from(err: GeomError) -> Self {
        Self::GraphicsError(err)
    }
}
