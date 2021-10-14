use super::utils::*;
use proc_macro_error::{abort, ResultExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Lit, LitStr, Result, Token};

#[derive(Debug)]
pub enum ParamAttr {
    Name(LitStr),
    Description(LitStr),
    Range(Lit, Lit),
    Default(Lit),
    Internal,
}

#[derive(Default, Debug)]
pub struct ParamAttrs {
    pub desc: Option<String>,
    pub name: Option<String>,
    pub range: Option<(Lit, Lit)>,
    pub internal: bool,
    pub default: Option<Lit>,

    pub doc_raw: Vec<Attribute>,
    pub other_raw: Vec<Attribute>,
}

impl Parse for ParamAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut result = Self::default();

        let raw_outer = input.call(Attribute::parse_outer)?;
        let mut param_raw = Vec::new();

        for attr in raw_outer {
            if attr.path.is_ident("doc") {
                result.doc_raw.push(attr);
            } else if attr.path.is_ident("param") {
                param_raw.push(attr);
            } else {
                result.other_raw.push(attr);
            }
        }

        result.desc = {
            let doc_comment = get_doc_comment(&result.doc_raw);
            process_doc_string(&doc_comment)
        };

        let parsed_args: Vec<ParamAttr> = param_raw
            .iter()
            .flat_map(|attr| {
                attr.parse_args_with(Punctuated::<ParamAttr, Token![,]>::parse_terminated)
                    .unwrap_or_abort()
            })
            .collect();

        for p in parsed_args {
            match p {
                ParamAttr::Name(litstr) => {
                    result.name = Some(litstr.value());
                }
                ParamAttr::Default(lit) => {
                    result.default = Some(lit);
                }
                ParamAttr::Description(litstr) => {
                    result.desc = Some(litstr.value());
                }
                ParamAttr::Range(start, end) => {
                    result.range = Some((start, end));
                }
                ParamAttr::Internal => {
                    result.internal = true;
                }
            }
        }

        Ok(result)
    }
}

impl Parse for ParamAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        match &*name_str {
            "default" => {
                let _eq: Token![=] = input.parse()?;
                let lit: Lit = input.parse()?;
                Ok(Self::Default(lit))
            }
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
            "range" => {
                let _eq: Token![=] = input.parse()?;
                let start: Lit = input.parse()?;
                let _sep: Token![..=] = input.parse()?;
                let end: Lit = input.parse()?;
                Ok(Self::Range(start, end))
            }
            "internal" => Ok(Self::Internal),

            _ => abort!(name, "unrecognized param attribute value"),
        }
    }
}
