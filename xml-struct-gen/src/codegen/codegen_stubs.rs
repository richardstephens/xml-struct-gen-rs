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
    root_props: Option<ElemProps>,
) -> TokenStream {
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

    let maybe_parse_document_impl = if let Some(root_props) = root_props {
        generate_parse_document(&root_props)
    } else {
        quote! {}
    };

    let parse_children_impl = generate_parse_children(&attr_fields, &elem_fields, v.has_text);

    quote! {
        #[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
        pub struct #sn {
            #(#attr_field_tokens)*
            pub misc_attrs: HashMap<(Option<String>, String), String>,
            #(#elem_field_tokens)*
            #maybe_val_field
        }

        impl #sn {
            #maybe_parse_document_impl
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

fn owned_name_to_match_arm(n: &OwnedName) -> TokenStream {
    let ns_m = n
        .namespace
        .as_ref()
        .map(|s| quote! { Some(#s) })
        .unwrap_or_else(|| quote! { None });
    let local_name = &n.local_name;
    quote! {(#ns_m, #local_name)}
}

fn attr_field_matchers(attr_fields: &Vec<AttrField>) -> Vec<TokenStream> {
    attr_fields
        .iter()
        .map(|field| {
            let match_arm = owned_name_to_match_arm(&field.xml_name);
            let target_var = format_ident!("{}", field.sanitized_name);
            quote! {
                #match_arm => {
                    n.#target_var = Some(attr.value);
                }
            }
        })
        .collect()
}

fn elem_matchers(elem_fields: &Vec<(Vec<OwnedName>, Ident, Ident)>) -> Vec<TokenStream> {
    elem_fields
        .iter()
        .map(|(field, var_name, ty_name)| {
            let last = field.last().unwrap();
            let match_arm = owned_name_to_match_arm(last);
            quote! {
                #match_arm => {
                    n.#var_name.push(#ty_name::parse_children(attributes, iter)?);
                }
            }
        })
        .collect()
}

fn generate_parse_document(elem_props: &ElemProps) -> TokenStream {
    let root = elem_props.child_stacks.iter().next().unwrap();
    let match_arm = owned_name_to_match_arm(root.last().clone().unwrap());
    quote! {
        pub fn parse_document<R: std::io::Read>(mut reader: R) -> Result<Self, XmlParseError>  {
            let mut parser = xml::EventReader::new(reader).into_iter();
            while let Some(event) = parser.next() {
                match event {
                    Ok(xml::reader::XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                        #match_arm => {
                            return Self::parse_children(attributes, &mut parser);
                        }
                        _ => {}
                    },
                _ => {}
                }
            };
            todo!()
        }
    }
}

fn generate_parse_children(
    attr_fields: &Vec<AttrField>,
    elem_fields: &Vec<(Vec<OwnedName>, Ident, Ident)>,
    has_text: bool,
) -> TokenStream {
    let attr_field_matchers = attr_field_matchers(attr_fields);
    let elem_matchers = elem_matchers(&elem_fields);

    let text_handler = if has_text {
        quote! {n.value = Some(val); }
    } else {
        quote! {
            return Err(XmlParseError::UnexpectedCharacters(XmlDocumentPosition::Unknown));
        }
    };
    quote! {
        fn parse_children<T: std::io::Read>(
            attrs: Vec<xml::attribute::OwnedAttribute>,
            iter: &mut xml::reader::Events<T>
        ) -> Result<Self, XmlParseError> {
            let mut n = Self::default();

            for attr in attrs.into_iter() {
                match (attr.name.namespace.as_deref(), attr.name.local_name.as_str()) {
                    #(#attr_field_matchers)*
                    (ns, name) => {
                        n.misc_attrs.insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                    }
                }
            }

            while let Some(e) = iter.next() {
                match e {
                    Ok(xml::reader::XmlEvent::StartElement { name, attributes, .. }) => {
                        match (name.namespace.as_deref(), name.local_name.as_str()) {
                            #(#elem_matchers)*
                            _ => {//TODO: handle unrecognised elements
                                todo!();
                            }

                        }
                    }
                    Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                        return Ok(n);
                    }
                    Ok(xml::reader::XmlEvent::Characters(val)) => {
                        #text_handler
                    }
                    Err(e) => {return Err(e.into())}
                    _ => {}
                }
            }
            return Err(XmlParseError::ExpectedEndElement(XmlDocumentPosition::Unknown));
        }
    }
}
