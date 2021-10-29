use core::ops::RangeInclusive;

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

/// Describes the type kind of the parameter.
///
/// This is used for setting parameters externally and/or deriving appropriate
/// UI controls in `nightgraph-ui`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParamKind {
    Int,
    Float,
    UInt,
    Bool,
    Unsupported,
}

/// Describes the range of appropriate values for numeric parameters.
///
/// Can be used to validate numeric assignment operations, or place bounds
/// on UI controls.
#[derive(Clone, Debug, PartialEq)]
pub enum ParamRange {
    Int(RangeInclusive<i64>),
    Float(RangeInclusive<f64>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sketch::*;
    use nightsketch_derive::sketch;

    /// Tests that params are named based on their struct field ident unless
    /// the #[param(name="___")] attr is passed
    #[test]
    fn param_naming() {
        #[sketch]
        struct SketchFieldName {
            from_field: bool,
        }

        let ts = SketchFieldName::default();
        let md = &ts.param_metadata()[0];
        assert_eq!(
            md.name, "from_field",
            "sketch parameter name was not correctly derived from field name"
        );

        #[sketch]
        struct SketchAttrName {
            #[param(name = "from_attr")]
            from_field: bool,
        }

        let ts = SketchAttrName::default();
        let md = &ts.param_metadata()[0];
        assert_eq!(
            md.name, "from_attr",
            "sketch parameter name was not correctly derived from #[param(name = ____)] attribute"
        );
    }

    /// Tests that param's descriptions are derived from their doc comments
    /// unless the #[param(description="___")] attr is passed
    #[test]
    fn param_description() {
        #[sketch]
        struct SketchDescComment {
            /// from doc comment
            doop: bool,
        }

        let ts = SketchDescComment::default();
        let md = &ts.param_metadata()[0];
        assert_eq!(
            md.description,
            Some("from doc comment"),
            "sketch parameter description was not correctly derived from doc comment"
        );

        #[sketch]
        struct SketchDescAttr {
            /// Desc from doc comment should not be used
            #[param(description = "from attr")]
            doop: bool,
        }

        let ts = SketchDescAttr::default();
        let md = &ts.param_metadata()[0];

        assert_eq!(
            md.description, Some("from attr"),
            "sketch parameter description was not correctly derived from #[param(description = ____)] attribute"
        );

        #[sketch]
        struct SketchDescNone {
            doop: bool,
        }

        let ts = SketchDescNone::default();
        let md = &ts.param_metadata()[0];

        assert_eq!(
            md.description, None,
            "sketch parameter description was not None as expected"
        );
    }

    /// Tests the #[param(range=X..=Y)] attr on both integral and floating
    /// point params
    #[test]
    fn param_range() {
        #[sketch]
        struct Sketch0 {
            #[param(range = 0..=20)]
            doop: u32,
        }
        let ts = Sketch0::default();
        let md = &ts.param_metadata()[0];

        assert_eq!(
            md.range, Some(ParamRange::Int(0..=20)),
            "sketch parameter int range was not correctly derived from #[param(range = ____)] attribute"
        );

        #[sketch]
        struct Sketch1 {
            #[param(range = 0.2..=20.)]
            doop: f32,
        }
        let ts = Sketch1::default();
        let md = &ts.param_metadata()[0];

        assert_eq!(
            md.range, Some(ParamRange::Float(0.2..=20.)),
            "sketch parameter float range was not correctly derived from #[param(range = ____)] attribute"
        );

        #[sketch]
        struct Sketch2 {
            doop: i32,
        }
        let ts = Sketch2::default();
        let md = &ts.param_metadata()[0];

        assert_eq!(
            md.range, None,
            "sketch parameter range was not None as expected"
        );
    }

    /// Checks that all supported types are recognized by the macro parser, and
    /// assigned the correct ParamKind
    #[test]
    fn param_types() {
        #[sketch]
        struct TestSketch {
            /// ParamDesc
            a: bool,
            b: i8,
            c: i16,
            d: i32,
            e: i64,
            f: i128,
            g: isize,
            h: u8,
            i: u16,
            j: u32,
            k: u64,
            l: u128,
            m: usize,
            n: f32,
            o: f64,
            p: String,
        }
        let ts = TestSketch::default();
        let md = &ts.param_metadata();

        for param_metadata in md {
            match param_metadata.name {
                "a" => {
                    assert_eq!(param_metadata.kind, ParamKind::Bool)
                }
                "b" | "c" | "d" | "e" | "f" | "g" => {
                    assert_eq!(
                        param_metadata.kind,
                        ParamKind::Int,
                        "{}",
                        param_metadata.name
                    )
                }
                "h" | "i" | "j" | "k" | "l" | "m" => {
                    assert_eq!(param_metadata.kind, ParamKind::UInt)
                }
                "n" | "o" => {
                    assert_eq!(param_metadata.kind, ParamKind::Float)
                }
                "p" => {
                    assert_eq!(param_metadata.kind, ParamKind::Unsupported)
                }
                _ => {
                    panic!("match arm should not be reached")
                }
            }
        }
    }
}
