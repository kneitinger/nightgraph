use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;

mod parse;
use parse::*;
mod codegen;
use codegen::*;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn sketch(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let sketch_struct: SketchStruct = parse_macro_input!(input);
    let name = &sketch_struct.name;
    let params = &sketch_struct.params;

    let struct_tokens = quote!( #sketch_struct  );
    //let impl_sketchmetadata = impl_sketchmetadata_tokens(&sketch_struct);
    let consts_sketch_meta = consts_sketch_meta_tokens(&sketch_struct);
    let impl_sketchaccess = impl_sketchaccess_tokens(name, params);
    let impl_default = impl_default_tokens(name, params);

    quote! (
        #consts_sketch_meta
        #struct_tokens
        #impl_default
        #impl_sketchaccess
    )
    .into()
}

#[derive(Debug)]
struct SketchListSubMod {
    use_item: syn::ItemUse,
    //mod_path: syn::Ident,
    //sketch_struct: syn::Ident,
}

#[derive(Debug)]
struct SketchListMod {
    sketches: Vec<SketchListSubMod>,
}

impl syn::parse::Parse for SketchListMod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parsing
        let vis: syn::Visibility = input.parse()?;
        let mod_token: syn::Token![mod] = input.parse()?;
        let name: syn::Ident = input.parse()?;
        let braced_content;
        syn::braced!(braced_content in input);
        let sketches_raw: syn::punctuated::Punctuated<SketchListSubMod, syn::Token![;]> =
            braced_content.parse_terminated(SketchListSubMod::parse)?;
        let sketches = sketches_raw.into_pairs().map(|p| p.into_value()).collect();

        Ok(Self { sketches })
    }
}

impl syn::parse::Parse for SketchListSubMod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        println!("in SketchListSubMod");
        let use_item: syn::ItemUse = input.parse()?;

        //let full_path = use_stmt.tree.
        //let mod_path = full_path.segments.

        println!("sketch item: {:?}", use_item);
        Ok(Self {
            use_item,
            //mod_path,
            //sketch_struct,
        })
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn sketchlist(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mod_item: SketchListMod = syn::parse(input).unwrap();
    println!("\nmod_item: {:?}\n", mod_item);

    quote!().into()
}
