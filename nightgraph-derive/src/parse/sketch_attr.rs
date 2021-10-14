use super::utils::*;
use proc_macro_error::{abort, ResultExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, LitStr, Result, Token};

#[derive(Default, Debug)]
pub struct SketchAttrs {
    pub doc_raw: Vec<Attribute>,
    pub other_raw: Vec<Attribute>,

    pub name: Option<String>,
    pub desc: Option<String>,
}

#[derive(Debug)]
pub enum SketchAttr {
    Name(LitStr),
    Description(LitStr),
}

impl Parse for SketchAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut result = SketchAttrs::default();

        let raw_outer = input.call(Attribute::parse_outer)?;

        let mut sketch_attrs = Vec::new();

        for attr in raw_outer {
            if attr.path.is_ident("doc") {
                result.doc_raw.push(attr);
            } else if attr.path.is_ident("sketch") {
                sketch_attrs.push(attr);
            } else {
                result.other_raw.push(attr);
            }
        }

        result.desc = {
            let doc_comment = get_doc_comment(&result.doc_raw);
            process_doc_string(&doc_comment)
        };

        let parsed_args: Vec<SketchAttr> = sketch_attrs
            .iter()
            .flat_map(|attr| {
                attr.parse_args_with(Punctuated::<SketchAttr, Token![,]>::parse_terminated)
                    .unwrap_or_abort()
            })
            .collect();

        for p in parsed_args {
            match p {
                SketchAttr::Name(litstr) => {
                    result.name = Some(litstr.value());
                }
                SketchAttr::Description(litstr) => {
                    result.desc = Some(litstr.value());
                }
            }
        }
        Ok(result)
    }
}

impl Parse for SketchAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        match &*name_str {
            "description" => {
                let _eq: Token![=] = input.parse()?;
                let lit: LitStr = input.parse()?;
                Ok(Self::Description(lit))
            }
            "name" => {
                let _eq: Token![=] = input.parse()?;
                let lit: LitStr = input.parse()?;
                Ok(Self::Name(lit))
            }
            _ => abort!(name, "unrecognized sketch attribute value"),
        }
    }
}
