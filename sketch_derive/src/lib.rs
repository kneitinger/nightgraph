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
    let impl_sketchaccess = impl_sketchaccess_tokens(name, params);
    let impl_default = impl_default_tokens(name, params);

    quote! (
        #struct_tokens
        #impl_default
        #impl_sketchaccess
    )
    .into()
}

#[proc_macro]
pub fn sketchlist(input: TokenStream) -> TokenStream {
    let sketches: SketchList = parse_macro_input!(input);
    let sketch_mod_use_stmts = sketch_mod_use_tokens(&sketches.sketches);
    let sketch_subcommand_enum = sketch_subcommand_enum_tokens(&sketches.sketches);
    quote!(#sketch_mod_use_stmts #sketch_subcommand_enum).into()
}
