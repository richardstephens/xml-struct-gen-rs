use crate::codegen::AssignedTypeMap;
use crate::codegen::stubs_write::gen_write_element;
use crate::common::elem_props::{AttrField, ElemProps};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use xml::name::OwnedName;

pub fn gen_el_struct(
    k: &Vec<OwnedName>,
    v: &ElemProps,
    assigned: &AssignedTypeMap,
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

    let parse_elem_impl = generate_parse_elem(k);

    let parse_children_impl = generate_parse_children(&attr_fields, &elem_fields, v.has_text);

    let name_consts = generate_name_consts(k);

    let write_element_impl = gen_write_element(&attr_fields, &elem_fields, v.has_text);

    quote! {
        #[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
        pub struct #sn {
            #(#attr_field_tokens)*
            pub misc_attrs: HashMap<(Option<String>, String), String>,
            #[serde(skip)]
            pub misc_content: Vec<xml::reader::XmlEvent>,
            #(#elem_field_tokens)*
            #maybe_val_field
        }

        impl #sn {
            #name_consts

            #maybe_parse_document_impl
            #parse_elem_impl
            #parse_children_impl

            #write_element_impl
        }

    }
}

fn opt_str_tokens(s: Option<&str>) -> TokenStream {
    match s {
        None => quote! { None },
        Some(s) => quote! { Some(#s) },
    }
}

fn generate_name_consts(p0: &Vec<OwnedName>) -> TokenStream {
    let last = p0.last().unwrap();

    let local_name = &last.local_name;

    let ns_const = opt_str_tokens(last.namespace.as_deref());
    let pfx_const = opt_str_tokens(last.prefix.as_deref());

    let xml_rs_name = match (last.namespace.as_deref(), last.prefix.as_deref()) {
        (Some(ns), Some(_pfx)) => {
            quote! {xml::name::Name::qualified(#local_name, #ns, #pfx_const)}
        }
        (None, Some(pfx)) => {
            quote! {xml::name::Name::prefixed(#local_name, #pfx)}
        }
        (None, None) => {
            quote! {xml::name::Name::local(#local_name) }
        }
        _ => {
            panic!("unsupported ns/pfx combo");
        }
    };

    quote! {
        const XML_LOCAL_NAME: &'static str = #local_name;
        const XML_NAMESPACE: Option<&'static str> = #ns_const;
        const XML_PREFIX: Option<&'static str> = #pfx_const;

        const XML_RS_NAME: xml::name::Name<'static > = #xml_rs_name;
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

fn generate_parse_document(_elem_props: &ElemProps) -> TokenStream {
    quote! {
        pub fn parse_document<R: std::io::Read>(mut reader: R) -> Result<Self, XmlParseError>  {
            let mut parser = xml::EventReader::new(reader).into_iter();
            let root_element = Self::parse_element(&mut parser)?;
            match parser.next() {
                Some(Ok(xml::reader::XmlEvent::EndDocument)) => {
                    Ok(root_element)
                }
                None => {
                    //should this be an error? might mean something has unexpectedly consumed the EndElement
                    Ok(root_element)
                }
                Some(Ok(e)) => Err(XmlParseError::ExpectedEof(e)),
                Some(Err(e)) => Err(e.into()),
            }
        }
    }
}

fn generate_parse_elem(elem_path: &Vec<OwnedName>) -> TokenStream {
    let elem = elem_path.last().clone().unwrap();
    let match_arm = owned_name_to_match_arm(elem);
    let err_text = format!("{} element", elem.local_name);
    quote! {
        pub fn parse_element<R: std::io::Read>(iter: &mut xml::reader::Events<R>) -> Result<Self, XmlParseError>  {
            while let Some(event) = iter.next() {
                return match event {
                    Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                        //TODO: should we capture the contents of the event?
                        continue;
                    }
                    Ok(xml::reader::XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    }) => {
                        match (name.namespace.as_deref(), name.local_name.as_str()) {
                            #match_arm => Self::parse_children(attributes, iter),
                            _ => Err(XmlParseError::UnexpectedElement(name)),
                        }
                    }
                    Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                    Err(e) => Err(e.into()),
                };
            };

            Err(XmlParseError::UnexpectedEof(#err_text))
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
            return Err(XmlParseError::UnexpectedCharacters(XmlDocumentReference::Unknown));
        }
    };
    quote! {
        fn parse_children<R: std::io::Read>(
            attrs: Vec<xml::attribute::OwnedAttribute>,
            iter: &mut xml::reader::Events<R>
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
                    Ok(xml::reader::XmlEvent::StartElement { name, attributes, namespace }) => {
                        match (name.namespace.as_deref(), name.local_name.as_str()) {
                            #(#elem_matchers)*
                            _ => {
                                let mut depth: usize = 1;
                                n.misc_content.push(xml::reader::XmlEvent::StartElement { name, attributes, namespace });
                                while let Some(e) = iter.next() {
                                    match e {
                                        Ok(xml::reader::XmlEvent::StartElement { name, attributes, namespace }) => {
                                            n.misc_content.push(xml::reader::XmlEvent::StartElement { name, attributes, namespace });
                                            depth += 1;
                                        }
                                        Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                            n.misc_content.push(xml::reader::XmlEvent::EndElement { name });
                                            depth -= 1;
                                            if depth == 0 {
                                                break;
                                            }
                                        }
                                        Ok(evt) => {
                                            n.misc_content.push(evt);
                                        }
                                        Err(e) => return Err(e.into()),
                                    }
                                };
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
            return Err(XmlParseError::ExpectedEndElement(XmlDocumentReference::Unknown));
        }
    }
}
