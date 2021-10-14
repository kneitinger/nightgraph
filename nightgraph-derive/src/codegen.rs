use crate::parse::*;
use quote::quote;
use std::str::FromStr;
use syn::Type;

pub fn impl_default_tokens(name: &syn::Ident, params: &[SketchParam]) -> proc_macro2::TokenStream {
    let param_def_tokens: Vec<proc_macro2::TokenStream> = params
        .iter()
        .map(|param| {
            let p_name = &param.name;
            let p_def = &param.param_attrs.default;

            if let Some(val) = p_def {
                quote! { #p_name : #val  }
            } else {
                quote! { #p_name : Default::default()  }
            }
        })
        .collect();

    quote!(
        impl Default for #name {
            fn default() -> Self {
                Self {
                   #(#param_def_tokens),*
                }
            }
        }
    )
}

pub fn impl_sketchaccess_tokens(
    name: &syn::Ident,
    params: &[SketchParam],
) -> proc_macro2::TokenStream {
    let fn_param_metadata = fn_param_metadata_tokens(params);

    let elem_id_access_tokens: Vec<proc_macro2::TokenStream> = ["i64", "u64", "f64", "bool"]
        .iter()
        .map(|ty_str| fn_type_ref_by_id_tokens(ty_str, params))
        .collect();

    quote!(
        impl SketchAccess for #name {
            #fn_param_metadata
            #(#elem_id_access_tokens)*
        }
    )
}

fn kind_string_from_ty_str(ty_str: &str) -> String {
    match ty_str {
        "f64" => "Float",
        "i64" => "Int",
        "u64" => "UInt",
        "bool" => "Bool",
        _ => "Unsupported",
    }
    .to_string()
}

/// Generates a function like
/// ```ignore
///   fn mut_bool_by_id(&mut self, id: u64) -> SketchResult<&mut bool> {
///     92039402 => Ok(self.foo),
///     ...
///   }
/// ```
/// for the supported sketch parameter types: `bool`, `u64`, `i64`, and `f64`.
fn fn_type_ref_by_id_tokens(ty_str: &str, params: &[SketchParam]) -> proc_macro2::TokenStream {
    let valids: Vec<proc_macro2::TokenStream> = params
        .iter()
        .filter(|p| {
            if let Some(s) = primitive_type_string(&p.ty) {
                s.as_str() == ty_str
            } else {
                false
            }
        })
        .map(|p| {
            let id = p.id;
            let name = &p.name;
            quote! { #id =>   Ok(&mut self.#name), }
        })
        .collect();

    let fn_ident = syn::Ident::new(
        &format!(
            "mut_{}_by_id",
            kind_string_from_ty_str(ty_str).to_lowercase()
        ),
        proc_macro2::Span::call_site(),
    );

    let ty = syn::Type::Verbatim(proc_macro2::TokenStream::from_str(ty_str).unwrap());

    let body = if valids.is_empty() {
        quote!(Err(SketchError::ConvertError))
    } else {
        quote!(
            match id {
                #(#valids)*
                _ => Err(SketchError::ConvertError),
            }
        )
    };

    quote!(
        fn #fn_ident(&mut self, id: u64) -> SketchResult<&mut #ty> {
            #body
        }
    )
}

fn fn_param_metadata_tokens(params: &[SketchParam]) -> proc_macro2::TokenStream {
    let mut param_metadata = vec![];
    for param in params {
        if !param.param_attrs.internal {
            param_metadata.push(param.generate_metadata_struct_literal());
        }
    }
    quote!(
        fn param_metadata(&self) -> Vec<ParamMetadata> {
            vec![
                #(#param_metadata),*
            ]
        }
    )
}

fn primitive_type_string(ty: &Type) -> Option<String> {
    use Type::*;
    match ty {
        Path(tp) => Some(tp.path.segments[0].ident.to_string()),
        Verbatim(ts) => Some(ts.to_string()),
        _ => None,
    }
}
