use proc_macro::{self, TokenStream};
use quote::quote;
use syn::*;

fn err<T: quote::ToTokens>(tokens: T, message: &str) -> TokenStream {
    Error::new_spanned(tokens, message)
        .to_compile_error()
        .into()
}

#[proc_macro_attribute]
pub fn sketch(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = parse_macro_input!(input);

    let sketch_struct = if let Item::Struct(ref struct_item) = item {
        struct_item
    } else {
        return err(&item, "#[sketch] can only be applied to structs");
    };

    let _sketch_name = &sketch_struct.ident;

    let output = quote! { #[derive(Sketch)] #item };
    output.into()
}
