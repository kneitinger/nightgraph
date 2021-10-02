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
}

#[derive(Subcommand, Serialize, Deserialize)]
pub enum Sketch {
    Blossom(Blossom),
}

impl Sketch {
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
}

trait SketchExec {
    fn exec(&self) -> SketchResult<Canvas>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
