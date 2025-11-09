use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub use xml_struct_types::v1::*;
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssDocument {
    pub r#version: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub channel_elems: Vec<Channel>,
}
impl RssDocument {
    const XML_LOCAL_NAME: &'static str = "rss";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("rss");
    pub fn parse_document<R: std::io::Read>(mut reader: R) -> Result<Self, XmlParseError> {
        let mut parser = xml::EventReader::new(reader).into_iter();
        while let Some(event) = parser.next() {
            match event {
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "rss") => {
                        return Self::parse_children(attributes, &mut parser);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        todo!()
    }
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "version") => {
                    n.r#version = Some(attr.value);
                }
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "channel") => {
                        n.channel_elems
                            .push(Channel::parse_children(attributes, iter)?);
                    }
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentPosition::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#version.as_ref() {
            el_builder = el_builder.attr("version", v);
        }
        w.write(el_builder)?;
        for child in self.channel_elems.iter() {
            child.write_element(w)?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Channel {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub rss_channel_title_elems: Vec<RssChannelTitle>,
    pub rss_channel_link_elems: Vec<RssChannelLink>,
    pub language_elems: Vec<Language>,
    pub copyright_elems: Vec<Copyright>,
    pub author_elems: Vec<Author>,
    pub rss_channel_description_elems: Vec<RssChannelDescription>,
    pub type_elems: Vec<Type>,
    pub rss_channel_itunes_image_elems: Vec<RssChannelItunesImage>,
    pub rss_channel_itunes_category_elems: Vec<RssChannelItunesCategory>,
    pub rss_channel_itunes_explicit_elems: Vec<RssChannelItunesExplicit>,
    pub item_elems: Vec<Item>,
}
impl Channel {
    const XML_LOCAL_NAME: &'static str = "channel";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("channel");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "title") => {
                        n.rss_channel_title_elems
                            .push(RssChannelTitle::parse_children(attributes, iter)?);
                    }
                    (None, "link") => {
                        n.rss_channel_link_elems
                            .push(RssChannelLink::parse_children(attributes, iter)?);
                    }
                    (None, "language") => {
                        n.language_elems
                            .push(Language::parse_children(attributes, iter)?);
                    }
                    (None, "copyright") => {
                        n.copyright_elems
                            .push(Copyright::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "author") => {
                        n.author_elems
                            .push(Author::parse_children(attributes, iter)?);
                    }
                    (None, "description") => {
                        n.rss_channel_description_elems
                            .push(RssChannelDescription::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "type") => {
                        n.type_elems.push(Type::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "image") => {
                        n.rss_channel_itunes_image_elems
                            .push(RssChannelItunesImage::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "category") => {
                        n.rss_channel_itunes_category_elems
                            .push(RssChannelItunesCategory::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "explicit") => {
                        n.rss_channel_itunes_explicit_elems
                            .push(RssChannelItunesExplicit::parse_children(attributes, iter)?);
                    }
                    (None, "item") => {
                        n.item_elems.push(Item::parse_children(attributes, iter)?);
                    }
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentPosition::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        for child in self.rss_channel_title_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.rss_channel_link_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.language_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.copyright_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.author_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.rss_channel_description_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.type_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.rss_channel_itunes_image_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.rss_channel_itunes_category_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.rss_channel_itunes_explicit_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.item_elems.iter() {
            child.write_element(w)?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelTitle {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelTitle {
    const XML_LOCAL_NAME: &'static str = "title";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("title");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelLink {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelLink {
    const XML_LOCAL_NAME: &'static str = "link";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("link");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Language {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Language {
    const XML_LOCAL_NAME: &'static str = "language";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("language");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Copyright {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Copyright {
    const XML_LOCAL_NAME: &'static str = "copyright";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("copyright");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Author {
    const XML_LOCAL_NAME: &'static str = "author";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "author",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelDescription {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelDescription {
    const XML_LOCAL_NAME: &'static str = "description";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("description");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Type {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Type {
    const XML_LOCAL_NAME: &'static str = "type";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "type",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesImage {
    pub r#href: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
}
impl RssChannelItunesImage {
    const XML_LOCAL_NAME: &'static str = "image";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "image",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "href") => {
                    n.r#href = Some(attr.value);
                }
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentPosition::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#href.as_ref() {
            el_builder = el_builder.attr("href", v);
        }
        w.write(el_builder)?;
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesCategory {
    pub r#text: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub rss_channel_itunes_category_itunes_category_elems:
        Vec<RssChannelItunesCategoryItunesCategory>,
}
impl RssChannelItunesCategory {
    const XML_LOCAL_NAME: &'static str = "category";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "category",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "text") => {
                    n.r#text = Some(attr.value);
                }
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "category") => {
                        n.rss_channel_itunes_category_itunes_category_elems.push(
                            RssChannelItunesCategoryItunesCategory::parse_children(
                                attributes, iter,
                            )?,
                        );
                    }
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentPosition::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#text.as_ref() {
            el_builder = el_builder.attr("text", v);
        }
        w.write(el_builder)?;
        for child in self
            .rss_channel_itunes_category_itunes_category_elems
            .iter()
        {
            child.write_element(w)?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesCategoryItunesCategory {
    pub r#text: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
}
impl RssChannelItunesCategoryItunesCategory {
    const XML_LOCAL_NAME: &'static str = "category";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "category",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "text") => {
                    n.r#text = Some(attr.value);
                }
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentPosition::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#text.as_ref() {
            el_builder = el_builder.attr("text", v);
        }
        w.write(el_builder)?;
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesExplicit {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItunesExplicit {
    const XML_LOCAL_NAME: &'static str = "explicit";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "explicit",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub rss_channel_item_itunes_title_elems: Vec<RssChannelItemItunesTitle>,
    pub rss_channel_item_link_elems: Vec<RssChannelItemLink>,
    pub rss_channel_item_itunes_image_elems: Vec<RssChannelItemItunesImage>,
    pub episode_type_elems: Vec<EpisodeType>,
    pub episode_elems: Vec<Episode>,
    pub season_elems: Vec<Season>,
    pub rss_channel_item_title_elems: Vec<RssChannelItemTitle>,
    pub rss_channel_item_description_elems: Vec<RssChannelItemDescription>,
    pub enclosure_elems: Vec<Enclosure>,
    pub guid_elems: Vec<Guid>,
    pub pub_date_elems: Vec<PubDate>,
    pub duration_elems: Vec<Duration>,
    pub rss_channel_item_itunes_explicit_elems: Vec<RssChannelItemItunesExplicit>,
}
impl Item {
    const XML_LOCAL_NAME: &'static str = "item";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("item");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "title") => {
                        n.rss_channel_item_itunes_title_elems
                            .push(RssChannelItemItunesTitle::parse_children(attributes, iter)?);
                    }
                    (None, "link") => {
                        n.rss_channel_item_link_elems
                            .push(RssChannelItemLink::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "image") => {
                        n.rss_channel_item_itunes_image_elems
                            .push(RssChannelItemItunesImage::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "episodeType") => {
                        n.episode_type_elems
                            .push(EpisodeType::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "episode") => {
                        n.episode_elems
                            .push(Episode::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "season") => {
                        n.season_elems
                            .push(Season::parse_children(attributes, iter)?);
                    }
                    (None, "title") => {
                        n.rss_channel_item_title_elems
                            .push(RssChannelItemTitle::parse_children(attributes, iter)?);
                    }
                    (None, "description") => {
                        n.rss_channel_item_description_elems
                            .push(RssChannelItemDescription::parse_children(attributes, iter)?);
                    }
                    (None, "enclosure") => {
                        n.enclosure_elems
                            .push(Enclosure::parse_children(attributes, iter)?);
                    }
                    (None, "guid") => {
                        n.guid_elems.push(Guid::parse_children(attributes, iter)?);
                    }
                    (None, "pubDate") => {
                        n.pub_date_elems
                            .push(PubDate::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "duration") => {
                        n.duration_elems
                            .push(Duration::parse_children(attributes, iter)?);
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "explicit") => {
                        n.rss_channel_item_itunes_explicit_elems.push(
                            RssChannelItemItunesExplicit::parse_children(attributes, iter)?,
                        );
                    }
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentPosition::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        for child in self.rss_channel_item_itunes_title_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.rss_channel_item_link_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.rss_channel_item_itunes_image_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.episode_type_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.episode_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.season_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.rss_channel_item_title_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.rss_channel_item_description_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.enclosure_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.guid_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.pub_date_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.duration_elems.iter() {
            child.write_element(w)?;
        }
        for child in self.rss_channel_item_itunes_explicit_elems.iter() {
            child.write_element(w)?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EpisodeType {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl EpisodeType {
    const XML_LOCAL_NAME: &'static str = "episodeType";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "episodeType",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemItunesTitle {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItemItunesTitle {
    const XML_LOCAL_NAME: &'static str = "title";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "title",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemDescription {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItemDescription {
    const XML_LOCAL_NAME: &'static str = "description";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("description");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Enclosure {
    pub r#length: Option<String>,
    pub r#type: Option<String>,
    pub r#url: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
}
impl Enclosure {
    const XML_LOCAL_NAME: &'static str = "enclosure";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("enclosure");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "length") => {
                    n.r#length = Some(attr.value);
                }
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "url") => {
                    n.r#url = Some(attr.value);
                }
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentPosition::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#length.as_ref() {
            el_builder = el_builder.attr("length", v);
        }
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#url.as_ref() {
            el_builder = el_builder.attr("url", v);
        }
        w.write(el_builder)?;
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Guid {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Guid {
    const XML_LOCAL_NAME: &'static str = "guid";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("guid");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PubDate {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl PubDate {
    const XML_LOCAL_NAME: &'static str = "pubDate";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("pubDate");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Duration {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Duration {
    const XML_LOCAL_NAME: &'static str = "duration";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "duration",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemItunesExplicit {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItemItunesExplicit {
    const XML_LOCAL_NAME: &'static str = "explicit";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "explicit",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Episode {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Episode {
    const XML_LOCAL_NAME: &'static str = "episode";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "episode",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Season {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Season {
    const XML_LOCAL_NAME: &'static str = "season";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "season",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemTitle {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItemTitle {
    const XML_LOCAL_NAME: &'static str = "title";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("title");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemItunesImage {
    pub r#href: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
}
impl RssChannelItemItunesImage {
    const XML_LOCAL_NAME: &'static str = "image";
    const XML_NAMESPACE: Option<&'static str> = Some("http://www.itunes.com/dtds/podcast-1.0.dtd");
    const XML_PREFIX: Option<&'static str> = Some("itunes");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "image",
        "http://www.itunes.com/dtds/podcast-1.0.dtd",
        Some("itunes"),
    );
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "href") => {
                    n.r#href = Some(attr.value);
                }
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentPosition::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#href.as_ref() {
            el_builder = el_builder.attr("href", v);
        }
        w.write(el_builder)?;
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemLink {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItemLink {
    const XML_LOCAL_NAME: &'static str = "link";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("link");
    fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (ns, name) => {
                    n.misc_attrs
                        .insert((ns.map(|s| s.to_string()), name.to_owned()), attr.value);
                }
            }
        }
        while let Some(e) = iter.next() {
            match e {
                Ok(xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentPosition::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> anyhow::Result<()> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
