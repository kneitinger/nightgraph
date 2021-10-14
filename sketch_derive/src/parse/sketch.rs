use super::{SketchAttrs, SketchParam};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Ident, Result, Token, Visibility};

#[derive(Debug)]
pub struct SketchStruct {
    pub sketch_attrs: SketchAttrs,

    pub vis: Visibility,
    pub name: Ident,

    pub params: Vec<SketchParam>,
}

impl Parse for SketchStruct {
    /// Parses a struct of the form:
    ///
    /// <doc comment/attrs>?            // via parse/sketch_attr.rs
    /// <pub>? struct SketchName {
    ///   (
    ///     <param doc comment/attrs>?  // via parse/param_attr.rs
    ///     <pub>? var: type,           // via parse/param.rs
    ///   )*
    /// }
    ///
    fn parse(input: ParseStream) -> Result<Self> {
        // Parsing
        let sketch_attrs: SketchAttrs = input.parse()?;
        let vis = input.parse()?;
        let _struct: Token![struct] = input.parse()?;
        let name = input.parse()?;

        let braced_content;
        braced!(braced_content in input);
        let params_raw: Punctuated<SketchParam, Token![,]> =
            braced_content.parse_terminated(SketchParam::parse)?;
        let params = params_raw.into_pairs().map(|p| p.into_value()).collect();

        Ok(Self {
            sketch_attrs,
            vis,
            name,
            params,
        })
    }
}

impl ToTokens for SketchStruct {
    fn to_tokens(&self, ts: &mut TokenStream) {
        let SketchStruct {
            ref sketch_attrs,
            ref vis,
            ref name,
            ref params,
        } = self;

        let SketchAttrs {
            ref doc_raw,
            ref other_raw,
            name: _,
            desc: _,
        } = sketch_attrs;

        let clap_attrs = if cfg!(feature = "cli") {
            Some(quote!(#[derive(clap::Clap)]))
        } else {
            None
        };
        let serde_attrs = if cfg!(feature = "serde_support") {
            Some(quote!(#[derive(serde::Deserialize, serde::Serialize)]))
        } else {
            None
        };

        // TODO: use name and desc to override clap name/desc if cli set
        let tokens = quote!(
            #(#doc_raw)*
            #(#other_raw)*
            #clap_attrs
            #serde_attrs
            #vis struct #name {
                #(#params),*
            }
        );
        tokens.to_tokens(ts);
    }
}
