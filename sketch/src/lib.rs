pub(crate) use nightgraphics::prelude::*;

mod sketch;
pub use sketch::*;
mod metadata;
pub use metadata::*;

mod sketches;
use sketches::*;

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
