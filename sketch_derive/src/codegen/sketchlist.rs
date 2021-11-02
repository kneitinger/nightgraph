use crate::parse::SketchListEntry;
use quote::quote;

pub fn sketch_subcommand_enum_tokens(sketches: &[SketchListEntry]) -> proc_macro2::TokenStream {
    // For each sketch, create an enum variant with its name and the struct
    // associated with it.  For example, if module `doop` has sketch `Doop`,
    // a variant `Doop(Doop)` is created.
    let sketch_enum_entries: Vec<proc_macro2::TokenStream> = sketches
        .iter()
        .map(|sketch| {
            let s = &sketch.sketch;
            quote!(#s(#s))
        })
        .collect();

    let exec_match_arms: Vec<proc_macro2::TokenStream> = sketches
        .iter()
        .map(|sketch| {
            let s = &sketch.sketch;
            quote!(Self::#s(s) => s.exec())
        })
        .collect();

    let serde_derive_attr = if cfg!(feature = "serde_support") {
        Some(quote!(#[derive(serde::Deserialize, serde::Serialize)]))
    } else {
        None
    };

    let sketch_subcommand_tokens = if cfg!(feature = "cli") {
        Some(quote! {
            #serde_derive_attr
            #[derive(clap::Parser)]
            pub enum SketchSubcommand {
                #(#sketch_enum_entries),*
            }

            impl SketchSubcommand {
                pub fn exec(&self) -> SketchResult<nightgraphics::prelude::Canvas> {
                    match self {
                        #(#exec_match_arms),*,
                        _ => Err(SketchError::Todo("TODO".to_string())),
                    }
                }
            }
        })
    } else {
        None
    };

    quote!( #sketch_subcommand_tokens )
}

pub fn sketch_mod_use_tokens(sketches: &[SketchListEntry]) -> proc_macro2::TokenStream {
    let mod_stmts: Vec<proc_macro2::TokenStream> = sketches
        .iter()
        .map(|sketch| {
            let m = &sketch.module;
            quote!(mod #m)
        })
        .collect();
    let use_stmts: Vec<proc_macro2::TokenStream> = sketches
        .iter()
        .map(|sketch| {
            let m = &sketch.module;
            let s = &sketch.sketch;
            quote!(pub use #m::#s)
        })
        .collect();

    quote!(
        #(#mod_stmts);*;
        #(#use_stmts);*;
    )
}
