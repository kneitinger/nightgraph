mod sketch;
pub use sketch::SketchStruct;
mod sketch_attr;
pub use sketch_attr::{SketchAttr, SketchAttrs};

mod param;
pub use param::SketchParam;
mod param_attr;
pub use param_attr::{ParamAttr, ParamAttrs};

mod sketchlist;
pub use sketchlist::{SketchList, SketchListEntry};

mod utils;

/*  /// Doc string
 *  #[sketch]
 *  #[other attribs]?
 *  pub struct <StuctName> {
 *      (
 *          #[param...]?                \
 *          #[other attrs]?             | - SketchParam
 *          <ParamNae>: <ParamType>     /
 *      ),*
 *  }
 */
