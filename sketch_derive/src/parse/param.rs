use super::utils::tokens_or_none;
use super::ParamAttrs;
use heck::KebabCase;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result, Token};

#[derive(Debug)]
pub struct SketchParam {
    pub param_attrs: ParamAttrs,
    pub name: Ident,
    pub ty: syn::Type,
    pub id: u64,
}

impl SketchParam {
    pub fn generate_metadata_struct_literal(&self) -> TokenStream {
        let Self {
            ref param_attrs,
            ref name,
            ref ty,
            ref id,
        } = self;

        let name_str = if let Some(n) = &param_attrs.name {
            n.to_owned()
        } else {
            name.to_string()
        };

        let desc = if let Some(desc) = &param_attrs.desc {
            quote!( Some(#desc) )
        } else {
            quote!(None)
        };

        let range = if let Some((start, end)) = &param_attrs.range {
            // The compiler will check if the type of `start` equals the type
            // of `end`, so only one needs to be checked for valid Lit types.
            match start {
                syn::Lit::Int(_) => quote!(Some(ParamRange::Int(#start..=#end))),
                syn::Lit::Float(_) => quote!(Some(ParamRange::Float(#start..=#end))),
                _ => abort!(start, "TODO: message about allowed types!"),
            }
        } else {
            quote!(None)
        };

        let kind = match ty {
            syn::Type::Path(tp) => match tp.path.segments[0].ident.to_string().as_str() {
                "isize" | "i128" | "i64" | "i32" | "i16" | "i8" => quote!(ParamKind::Int),
                "usize" | "u128" | "u64" | "u32" | "u16" | "u8" => quote!(ParamKind::UInt),
                "f64" | "f32" => quote!(ParamKind::Float),
                "bool" => quote!(ParamKind::Bool),
                _ => quote!(ParamKind::Unsupported),
            },
            _ => quote!(ParamKind::Unsupported),
        };

        let default_val = match &param_attrs.default {
            Some(lit) => quote!(#lit),
            None => quote!(Default::default()),
        };

        let default = match ty {
            syn::Type::Path(tp) => match tp.path.segments[0].ident.to_string().as_str() {
                "isize" | "i128" | "i64" | "i32" | "i16" | "i8" => {
                    quote!(ParamDefault::Int(#default_val))
                }
                "usize" | "u128" | "u64" | "u32" | "u16" | "u8" => {
                    quote!(ParamDefault::UInt(#default_val))
                }
                "f64" | "f32" => {
                    quote!(ParamDefault::Float(#default_val))
                }
                "bool" => {
                    quote!(ParamDefault::Bool(#default_val))
                }
                _ => quote!(ParamDefault::None),
            },
            _ => quote!(ParamDefault::None),
        };

        quote!(ParamMetadata { id: #id, name: #name_str, description: #desc, kind: #kind, range: #range, default: #default})
    }

    fn generate_secondary_attrs(&self) -> Option<TokenStream> {
        let skip_tokens = self.generate_skip_attr_tokens();
        let clap_tokens = self.generate_clap_attr_tokens();

        Some(quote!(#skip_tokens #clap_tokens ))
    }
    fn generate_skip_attr_tokens(&self) -> Option<TokenStream> {
        let clap_skip = tokens_or_none(cfg!(feature = "cli"), quote! { #[clap(skip)] });

        let serde_skip = tokens_or_none(cfg!(feature = "serde_support"), quote! { #[serde(skip)] });

        tokens_or_none(self.param_attrs.internal, quote! {#clap_skip #serde_skip})
    }

    fn generate_clap_attr_tokens(&self) -> Option<TokenStream> {
        if !cfg!(feature = "cli") || self.param_attrs.internal {
            return None;
        }

        let param_name = self
            .param_attrs
            .name
            .as_ref()
            .unwrap_or(&self.name.to_string())
            .to_kebab_case();

        // Handle clap default vals
        match &self.param_attrs.default {
            None => Some(quote! { #[clap(long)] }),
            Some(lit_def) => {
                if let syn::Lit::Bool(litbool) = lit_def {
                    if litbool.value() {
                        // TODO: intelligently handle clap short name conflicts (needs
                        // to happen earlier in the processing, since all vars need to
                        // be observed to detect conflicts
                        let negated_param_name = format!("no-{}", param_name);
                        Some(
                            quote! { #[clap(long=#negated_param_name, parse(from_flag = std::ops::Not::not))] },
                        )
                    } else {
                        Some(quote! { #[clap(long=#param_name)] })
                    }
                } else {
                    // Non-bool
                    Some(quote! { #[clap(long=#param_name, default_value_t=#lit_def)] })
                }
            }
        }
    }
}

impl Parse for SketchParam {
    fn parse(input: ParseStream) -> Result<Self> {
        // Eventually use parse_sketch_param_attributes() here
        //let raw_attrs = input.call(syn::Attribute::parse_outer)?;
        let param_attrs: ParamAttrs = input.parse()?;

        let name: Ident = input.parse()?;
        let _colon: Token![:] = input.parse()?;
        let ty: syn::Type = input.parse()?;

        let name_str = name.to_string();
        let id = {
            let mut s = DefaultHasher::new();
            name_str.hash(&mut s);
            s.finish()
        };

        Ok(Self {
            param_attrs,
            name,
            ty,
            id,
        })
    }
}

impl ToTokens for SketchParam {
    fn to_tokens(&self, ts: &mut TokenStream) {
        let SketchParam {
            ref param_attrs,
            ref name,
            ref ty,
            id: _,
            //ref raw_doc_comment,
        } = self;

        let doc_attrs = &param_attrs.doc_raw;
        let other_attrs = &param_attrs.other_raw;

        let secondary_attrs = self.generate_secondary_attrs();
        let tokens = quote! { #(#doc_attrs)* #(#other_attrs)* #secondary_attrs #name: #ty };

        tokens.to_tokens(ts);
    }
}
