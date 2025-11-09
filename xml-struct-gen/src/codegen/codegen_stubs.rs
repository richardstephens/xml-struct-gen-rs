use crate::common::elem_props::{AttrField, ElemProps};
use bimap::BiMap;
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use xml::name::OwnedName;

pub fn gen_el_struct(
    k: &Vec<OwnedName>,
    v: &ElemProps,
    assigned: &BiMap<Vec<OwnedName>, String>,
) -> TokenStream {
    if k.is_empty() {
        return quote! {};
    }

    let attr_fields = v.get_attr_fields();
    let attr_field_tokens: Vec<TokenStream> = attr_fields_to_tokens(&attr_fields);

    let elem_fields: Vec<_> = v
        .child_stacks
        .iter()
        .map(|x| {
            let var_name =
                format_ident!("{}_elems", assigned.get_by_left(x).unwrap().to_snake_case());
            let ty_name = format_ident!("{}", assigned.get_by_left(x).unwrap());
            (x.clone(), var_name, ty_name)
        })
        .collect();
    let elem_field_tokens: Vec<TokenStream> = elem_fields_to_tokens(&elem_fields);

    let maybe_val_field = match v.has_text {
        true => quote! {
            pub value: Option<String>,
        },
        false => quote! {},
    };

    let xml_name = format_ident!("{}", assigned.get_by_left(k).unwrap());
    let sn = if v.is_root {
        format_ident!("{xml_name}Document")
    } else {
        format_ident!("{xml_name}")
    };

    quote! {
        #[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
        pub struct #sn {
            #(#attr_field_tokens)*
            #(#elem_field_tokens)*
            #maybe_val_field
        }
    }
}

fn elem_fields_to_tokens(elem_fields: &Vec<(Vec<OwnedName>, Ident, Ident)>) -> Vec<TokenStream> {
    elem_fields
        .iter()
        .map(|(_, var_name, ty_name)| quote! {pub #var_name: Vec<#ty_name>,})
        .collect()
}

fn attr_fields_to_tokens(attr_fields: &Vec<AttrField>) -> Vec<TokenStream> {
    attr_fields
        .iter()
        .map(|x| {
            let field_ident = format_ident!("{}", &x.sanitized_name);
            quote! {
                pub #field_ident: Option<String>,
            }
        })
        .collect()
}
