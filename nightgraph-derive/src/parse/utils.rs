use proc_macro2::TokenStream;
use syn::Attribute;

pub fn tokens_or_none(condition: bool, tokens: TokenStream) -> Option<TokenStream> {
    if condition {
        Some(tokens)
    } else {
        None
    }
}

pub fn get_doc_comment(attrs: &[Attribute]) -> Vec<String> {
    use syn::Lit::*;
    use syn::Meta::*;
    use syn::MetaNameValue;

    let comment_parts: Vec<_> = attrs
        .iter()
        .filter(|attr| attr.path.is_ident("doc"))
        .filter_map(|attr| {
            if let Ok(NameValue(MetaNameValue { lit: Str(s), .. })) = attr.parse_meta() {
                Some(s.value())
            } else {
                // non #[doc = "..."] attributes are not our concern
                // we leave them for rustc to handle
                None
            }
        })
        .collect();

    comment_parts
}

/// Turns the vector of Strings (one per line in doc comment) into a single,
/// nicely formatted string to use as a sketch/param description
pub fn process_doc_string(doc_lines: &[String]) -> Option<String> {
    // strip any lines that are empty
    let mut lines: Vec<&str> = doc_lines
        .iter()
        .skip_while(|s| s.trim().is_empty())
        .map(|s| s.as_str())
        .collect();

    while let Some(true) = lines.last().map(|s| s.trim().is_empty()) {
        lines.pop();
    }

    if lines.is_empty() {
        return None;
    }

    let mut s = String::new();
    for line in lines {
        s.push_str(line);
    }

    s.split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .reduce(|a, b| format!("{} {}", a, b))
}
