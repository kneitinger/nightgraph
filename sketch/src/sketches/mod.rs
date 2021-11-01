mod blossom;

use super::*;
pub use blossom::Blossom;
pub(self) use nightsketch_derive::sketch;

#[cfg_attr(feature = "cli", derive(clap::Parser))]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, serde::Serialize)
)]
pub enum SketchSubcommand {
    Blossom(Blossom),
}

impl Default for SketchSubcommand {
    fn default() -> Self {
        Self::Blossom(Blossom::default())
    }
}

pub struct SketchList {}

impl SketchList {
    pub fn default_sketch() -> Box<dyn Sketch> {
        Box::new(Blossom::default())
    }
    pub fn sketch_by_name(name: &str) -> SketchResult<Box<dyn Sketch>> {
        match name {
            "blossom" => Ok(Box::new(Blossom::default())),
            _ => Err(SketchError::Todo("sdfs".to_string())),
        }
    }
}
