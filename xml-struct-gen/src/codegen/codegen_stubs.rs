use crate::struct_scan::elem_props::ElemProps;
use bimap::BiMap;
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
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
    let attr_fields: Vec<_> = v
        .attributes
        .iter()
        .map(|x| x.local_name.clone())
        .map(|x| {
            let field_ident = format_ident!("{}", sanitize_field_name(&x));
            quote! {
                pub #field_ident: Option<String>,
            }
        })
        .collect();

    let elem_fields: Vec<_> = v
        .child_stacks
        .iter()
        .map(|x| {
            let ty_name = format_ident!("{}", assigned.get_by_left(x).unwrap());
            let var_name =
                format_ident!("{}_elems", assigned.get_by_left(x).unwrap().to_snake_case());
            quote! {pub #var_name: Vec<#ty_name>,}
        })
        .collect();

    let maybe_val_field = match v.has_text {
        true => quote! {
            pub value: Option<String>,
        },
        false => quote! {},
    };

    let xml_name = format_ident!("{}", assigned.get_by_left(k).unwrap());
    let sn = format_ident!("{xml_name}");

    quote! {
        #[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
        pub struct #sn {
            #(#attr_fields)*
            #(#elem_fields)*
            #maybe_val_field
        }
    }
}

fn sanitize_field_name(name: &str) -> String {
    format!("r#{}", name.to_snake_case())
}
