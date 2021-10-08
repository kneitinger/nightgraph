pub(crate) use clap::Clap;
use clap::Subcommand;
pub(crate) use nightgraphics::prelude::*;
pub(crate) use serde::{Deserialize, Serialize};

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

#[derive(Subcommand, Serialize, Deserialize)]
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

#[derive(Copy, Clone)]
pub enum UiHint {
    Slider,
}

#[derive(Copy, Clone)]
pub struct Param {
    id: u8,
    pub name: &'static str,
    pub description: &'static str,
    pub kind: ParamKind,
    pub ui_hint: Option<UiHint>,
}

impl Param {
    pub fn id(&self) -> u8 {
        self.id
    }
    pub fn kind(&self) -> ParamKind {
        self.kind
    }
}

trait ParamCast {
    fn to_param(kind: ParamKind);
}

impl SketchList {
    fn inner_sketch(&self) -> &dyn SketchExec {
        match self {
            Self::Blossom(s) => s as &dyn SketchExec,
        }
    }
    fn inner_sketch_mut(&mut self) -> &mut dyn SketchExec {
        match self {
            Self::Blossom(s) => s as &mut dyn SketchExec,
        }
    }
    pub fn exec(&self) -> SketchResult<Canvas> {
        self.inner_sketch().exec()
    }
    pub fn params(&self) -> Vec<Param> {
        self.inner_sketch().params()
    }
    pub fn mut_float_by_id(&mut self, id: u8) -> SketchResult<&mut f64> {
        self.inner_sketch_mut().mut_float_by_id(id)
    }
    pub fn mut_int_by_id(&mut self, id: u8) -> SketchResult<&mut i64> {
        self.inner_sketch_mut().mut_int_by_id(id)
    }
    pub fn mut_uint_by_id(&mut self, id: u8) -> SketchResult<&mut u64> {
        self.inner_sketch_mut().mut_uint_by_id(id)
    }

    pub fn mut_bool_by_id(&mut self, id: u8) -> SketchResult<&mut bool> {
        self.inner_sketch_mut().mut_bool_by_id(id)
    }
}

trait SketchExec {
    fn params(&self) -> Vec<Param>;
    fn mut_float_by_id(&mut self, id: u8) -> SketchResult<&mut f64>;
    fn mut_int_by_id(&mut self, id: u8) -> SketchResult<&mut i64>;
    fn mut_uint_by_id(&mut self, id: u8) -> SketchResult<&mut u64>;
    fn mut_bool_by_id(&mut self, id: u8) -> SketchResult<&mut bool>;

    fn get_kind_by_id(&mut self, id: u8) -> SketchResult<ParamKind> {
        Ok(self
            .params()
            .iter()
            .find(|p| p.id == id)
            .ok_or_else(|| SketchError::ParamError(format!("Invalid id: {}", id)))?
            .kind)
    }
    fn exec(&self) -> SketchResult<Canvas>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_param_set_types() {
        let mut sketch = SketchList::Blossom(Blossom::default());
        let params = sketch.params();

        assert_eq!(2 + 2, 4);
    }
}
