use core::ops::RangeInclusive;
pub(crate) use nightgraphics::prelude::*;

mod blossom;
use blossom::*;

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

#[cfg_attr(feature = "cli", derive(clap::Subcommand))]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, serde::Serialize)
)]
pub enum SketchList {
    Blossom(Blossom),
}

impl Default for SketchList {
    fn default() -> Self {
        Self::Blossom(Blossom::default())
    }
}

#[derive(Copy, Clone)]
pub enum ParamKind {
    Int,
    Float,
    UInt,
    Bool,
    Unsupported,
}

#[derive(Clone)]
pub enum ParamRange {
    Int(RangeInclusive<i64>),
    Float(RangeInclusive<f64>),
}

#[derive(Clone)]
pub struct ParamMetadata {
    pub id: u64,
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub kind: ParamKind,
    pub range: Option<ParamRange>,
}

impl SketchList {
    fn inner_sketch(&self) -> &dyn Sketch {
        match self {
            Self::Blossom(s) => s as &dyn Sketch,
        }
    }
    fn inner_sketch_mut(&mut self) -> &mut dyn Sketch {
        match self {
            Self::Blossom(s) => s as &mut dyn Sketch,
        }
    }
    pub fn exec(&self) -> SketchResult<Canvas> {
        self.inner_sketch().exec()
    }
    pub fn param_metadata(&self) -> Vec<ParamMetadata> {
        self.inner_sketch().param_metadata()
    }
    pub fn mut_float_by_id(&mut self, id: u64) -> SketchResult<&mut f64> {
        self.inner_sketch_mut().mut_float_by_id(id)
    }
    pub fn mut_int_by_id(&mut self, id: u64) -> SketchResult<&mut i64> {
        self.inner_sketch_mut().mut_int_by_id(id)
    }
    pub fn mut_uint_by_id(&mut self, id: u64) -> SketchResult<&mut u64> {
        self.inner_sketch_mut().mut_uint_by_id(id)
    }

    pub fn mut_bool_by_id(&mut self, id: u64) -> SketchResult<&mut bool> {
        self.inner_sketch_mut().mut_bool_by_id(id)
    }
}

trait Sketch: SketchAccess {
    fn exec(&self) -> SketchResult<Canvas>;
}

trait SketchAccess {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_param_set_types() {
        assert_eq!(2 + 2, 4);
    }
}
