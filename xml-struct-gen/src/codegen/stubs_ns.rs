use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;

pub fn generate_namespace_const(ns: BTreeMap<String, String>) -> TokenStream {
    let entries: Vec<TokenStream> = ns
        .iter()
        .filter(|(k, v)| !k.is_empty() && !v.is_empty())
        .filter(|(k, _)| k.as_str() != "xml" && k.as_str() != "xmlns")
        .map(|(k, v)| {
            quote! { (#k, #v), }
        })
        .collect();

    quote! {
        const DOCUMENT_NAMESPACES: &[(&str, &str)] = &[#(#entries)*];
    }
}
