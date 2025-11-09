use crate::common::elem_props::AttrField;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn gen_write_element(attr_fields: &Vec<AttrField>) -> TokenStream {
    let attr_write_tokens = attr_field_readers(attr_fields);
    quote! {
        pub fn write_element<W: std::io::Write>(&self, w: &mut xml::writer::EventWriter<W>) -> anyhow::Result<()> {

            let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
            #(#attr_write_tokens)*
            // TODO attrs
            w.write(el_builder)?;

            //TODO write children
            w.write(xml::writer::XmlEvent::end_element())?;
            Ok(())
        }
    }
}

fn attr_field_readers(fields: &Vec<AttrField>) -> Vec<TokenStream> {
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
