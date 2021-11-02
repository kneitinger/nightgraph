use super::*;
pub(self) use nightsketch_derive::{sketch, sketchlist};

pub struct SketchList {}

impl SketchList {
    pub fn default_sketch() -> Box<dyn Sketch> {
        Box::new(blossom::Blossom::default())
    }
    pub fn sketch_by_name(name: &str) -> SketchResult<Box<dyn Sketch>> {
        match name {
            "blossom" => Ok(Box::new(blossom::Blossom::default())),
            _ => Err(SketchError::Todo("sdfs".to_string())),
        }
    }
}

sketchlist! {
    blossom::Blossom,
}
