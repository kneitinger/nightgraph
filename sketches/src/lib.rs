pub(crate) use clap::Clap;
use clap::Subcommand;
pub(crate) use nightgraphics::prelude::*;
pub(crate) use serde::{Deserialize, Serialize};

mod blossom;
use blossom::*;

pub type SketchResult = Result<Canvas, SketchError>;

#[derive(Debug)]
pub enum SketchError {
    Todo(String),
}

#[derive(Subcommand, Serialize, Deserialize)]
pub enum Sketch {
    Blossom(Blossom),
}

impl Sketch {
    pub fn exec(&self) -> SketchResult {
        match self {
            Self::Blossom(s) => s.exec(),
        }
    }
}

trait SketchExec {
    fn exec(&self) -> SketchResult;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
