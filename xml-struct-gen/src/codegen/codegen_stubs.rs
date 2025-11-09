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

    let parse_children_impl = generate_parse_children(&attr_fields);

    quote! {
        #[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
        pub struct #sn {
            #(#attr_field_tokens)*
            pub misc_attrs: HashMap<(Option<String>, String), String>,
            #(#elem_field_tokens)*
            #maybe_val_field
        }

        impl #sn {
            #parse_children_impl
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

fn attr_field_matchers(attr_fields: &Vec<AttrField>) -> Vec<TokenStream> {
    attr_fields
        .iter()
        .map(|field| {
            let ns_m = field
                .xml_name
                .namespace
                .as_ref()
                .map(|s| quote! { Some(#s) })
                .unwrap_or_else(|| quote! { None });
            let local_name = &field.xml_name.local_name;
            let target_var = format_ident!("{}", field.sanitized_name);
            quote! {
                (#ns_m, #local_name) => {
                    n.#target_var = Some(attr.value);
                }
            }
        })
        .collect()
}

fn generate_parse_children(attr_fields: &Vec<AttrField>) -> TokenStream {
    let attr_field_matchers = attr_field_matchers(attr_fields);

    quote! {
       pub fn parse_children<T: std::io::Read>(
            attrs: Vec<xml::attribute::OwnedAttribute>,
            iter: &mut xml::reader::Events<T>
        ) -> Self {
            let mut n = Self::default();

            for attr in attrs.into_iter() {
                match (attr.name.namespace.as_deref(), attr.name.local_name.as_str()) {
                    #(#attr_field_matchers)*
                    (ns, name) => {
                        n.misc_attrs.insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                    }
                }
            }
            n
        }
    }
}
