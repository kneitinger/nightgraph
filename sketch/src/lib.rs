use core::ops::RangeInclusive;
pub(crate) use nightgraphics::prelude::*;
pub(crate) use nightsketch_derive::sketch;

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

#[cfg_attr(feature = "cli", derive(clap::Parser))]
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

/// Information associated with a sketch parameter.
///
/// This metadata can be used to set sketch parameter values, generate controls
/// in other crates, etc.
#[derive(Clone)]
pub struct ParamMetadata {
    /// A unique id that can be used in the mut accessor funtions
    pub id: u64,
    /// The name of the parameter
    pub name: &'static str,
    /// A description of the parameter
    pub description: Option<&'static str>,
    /// An enum representing the supported kind of parameter; or if it is
    /// unsupported
    pub kind: ParamKind,
    /// The range of appropriate values for this parameter. Only meaningful
    /// for numeric types
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
mod tests {}
