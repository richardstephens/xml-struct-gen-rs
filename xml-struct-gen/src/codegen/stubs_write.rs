use crate::common::elem_props::AttrField;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use xml::name::OwnedName;

pub fn gen_write_element(
    attr_fields: &Vec<AttrField>,
    elem_fields: &Vec<(Vec<OwnedName>, Ident, Ident)>,
    has_text: bool,
) -> TokenStream {
    let attr_write_tokens = attr_writers(attr_fields);
    let elem_write_tokens = elem_writers(elem_fields);
    let text_write_tokens = if has_text {
        quote! {
            if let Some(val) = self.value.as_deref() {
                w.write(xml::writer::XmlEvent::characters(val))?;
            }
        }
    } else {
        quote! {}
    };

    quote! {
        pub fn write_element<W: std::io::Write>(&self, w: &mut xml::writer::EventWriter<W>) -> anyhow::Result<()> {

            let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
            #(#attr_write_tokens)*
            w.write(el_builder)?;

            #(#elem_write_tokens)*

            #text_write_tokens

            w.write(xml::writer::XmlEvent::end_element())?;
            Ok(())
        }
    }
}

fn elem_writers(elem_fields: &Vec<(Vec<OwnedName>, Ident, Ident)>) -> Vec<TokenStream> {
    elem_fields
        .iter()
        .map(|(_, var_name, _)| {
            quote! {
                for child in self.#var_name.iter() {
                    child.write_element(w)?;
                }
            }
        })
        .collect()
}

fn attr_writers(fields: &Vec<AttrField>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let ident = format_ident!("{}", field.sanitized_name);

            //TODO this should do proper namespace support
            let local_name = &field.xml_name.local_name;

            quote! {
                if let Some(v) = self.#ident.as_ref() {
                    el_builder = el_builder.attr(#local_name, v);
                }
            }
        })
        .collect()
}
