use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Path, Result, Token};

#[derive(Debug)]
pub struct SketchListEntry {
    pub module: Punctuated<syn::PathSegment, Token![::]>,
    pub sketch: Ident,
}

impl Parse for SketchListEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let path: syn::Path = input.call(Path::parse_mod_style)?;
        let mut seg_vec: Vec<syn::PathSegment> = path.segments.into_iter().collect();
        let sketch = seg_vec.pop().unwrap().ident;
        let mut module = Punctuated::new();
        for seg in seg_vec {
            module.push(seg);
        }
        Ok(SketchListEntry { module, sketch })
    }
}

#[derive(Debug)]
pub struct SketchList {
    pub sketches: Vec<SketchListEntry>,
}

impl Parse for SketchList {
    fn parse(input: ParseStream) -> Result<Self> {
        let sketches: Punctuated<SketchListEntry, Token![,]> =
            input.parse_terminated(SketchListEntry::parse)?;
        let sketches: Vec<SketchListEntry> = sketches.into_iter().collect();

        Ok(SketchList { sketches })
    }
}
