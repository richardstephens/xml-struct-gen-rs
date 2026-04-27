use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub use xml_struct_types::v1::error::*;
const DOCUMENT_NAMESPACES: &[(&str, &str)] =
    &[("libosinfo", "http://libosinfo.org/xmlns/libvirt/domain/1.0")];
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDocument {
    pub r#type: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub uuid_elems: Vec<Uuid>,
    pub title_elems: Vec<Title>,
    pub description_elems: Vec<Description>,
    pub metadata_elems: Vec<Metadata>,
    pub on_poweroff_elems: Vec<OnPoweroff>,
    pub on_reboot_elems: Vec<OnReboot>,
    pub on_crash_elems: Vec<OnCrash>,
    pub name_elems: Vec<Name>,
    pub memory_elems: Vec<Memory>,
    pub current_memory_elems: Vec<CurrentMemory>,
    pub vcpu_elems: Vec<Vcpu>,
    pub domain_os_elems: Vec<DomainOs>,
    pub sysinfo_elems: Vec<Sysinfo>,
    pub features_elems: Vec<Features>,
    pub clock_elems: Vec<Clock>,
    pub cpu_elems: Vec<Cpu>,
    pub pm_elems: Vec<Pm>,
    pub devices_elems: Vec<Devices>,
}
impl DomainDocument {
    const XML_LOCAL_NAME: &'static str = "domain";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("domain");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "domain") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("domain element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "uuid") => {
                        n.uuid_elems.push(Uuid::parse_children(attributes, iter)?);
                    }
                    (None, "title") => {
                        n.title_elems.push(Title::parse_children(attributes, iter)?);
                    }
                    (None, "description") => {
                        n.description_elems
                            .push(Description::parse_children(attributes, iter)?);
                    }
                    (None, "metadata") => {
                        n.metadata_elems
                            .push(Metadata::parse_children(attributes, iter)?);
                    }
                    (None, "on_poweroff") => {
                        n.on_poweroff_elems
                            .push(OnPoweroff::parse_children(attributes, iter)?);
                    }
                    (None, "on_reboot") => {
                        n.on_reboot_elems
                            .push(OnReboot::parse_children(attributes, iter)?);
                    }
                    (None, "on_crash") => {
                        n.on_crash_elems
                            .push(OnCrash::parse_children(attributes, iter)?);
                    }
                    (None, "name") => {
                        n.name_elems.push(Name::parse_children(attributes, iter)?);
                    }
                    (None, "memory") => {
                        n.memory_elems
                            .push(Memory::parse_children(attributes, iter)?);
                    }
                    (None, "currentMemory") => {
                        n.current_memory_elems
                            .push(CurrentMemory::parse_children(attributes, iter)?);
                    }
                    (None, "vcpu") => {
                        n.vcpu_elems.push(Vcpu::parse_children(attributes, iter)?);
                    }
                    (None, "os") => {
                        n.domain_os_elems
                            .push(DomainOs::parse_children(attributes, iter)?);
                    }
                    (None, "sysinfo") => {
                        n.sysinfo_elems
                            .push(Sysinfo::parse_children(attributes, iter)?);
                    }
                    (None, "features") => {
                        n.features_elems
                            .push(Features::parse_children(attributes, iter)?);
                    }
                    (None, "clock") => {
                        n.clock_elems.push(Clock::parse_children(attributes, iter)?);
                    }
                    (None, "cpu") => {
                        n.cpu_elems.push(Cpu::parse_children(attributes, iter)?);
                    }
                    (None, "pm") => {
                        n.pm_elems.push(Pm::parse_children(attributes, iter)?);
                    }
                    (None, "devices") => {
                        n.devices_elems
                            .push(Devices::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.uuid_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.title_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.description_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.metadata_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.on_poweroff_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.on_reboot_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.on_crash_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.name_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.memory_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.current_memory_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.vcpu_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_os_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.sysinfo_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.features_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.clock_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.cpu_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.pm_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.devices_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
impl xml_struct_types::v1::XmlStructDocument for DomainDocument {
    fn parse_document<R: std::io::Read>(reader: &mut R) -> Result<Self, XmlParseError> {
        let mut parser = xml::EventReader::new(reader).into_iter();
        let root_element = Self::parse_element(&mut parser)?;
        match parser.next() {
            Some(Ok(xml::reader::XmlEvent::EndDocument)) => Ok(root_element),
            None => Ok(root_element),
            Some(Ok(e)) => Err(XmlParseError::ExpectedEof(e)),
            Some(Err(e)) => Err(e.into()),
        }
    }
    fn write_document<W: std::io::Write>(&self, w: &mut W) -> Result<(), XmlWriteError> {
        let mut writer = xml::EmitterConfig::new()
            .write_document_declaration(false)
            .perform_indent(false)
            .create_writer(w);
        self.write_element(&mut writer, true)
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Name {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Name {
    const XML_LOCAL_NAME: &'static str = "name";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("name");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "name") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("name element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Uuid {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Uuid {
    const XML_LOCAL_NAME: &'static str = "uuid";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("uuid");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "uuid") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("uuid element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Title {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Title {
    const XML_LOCAL_NAME: &'static str = "title";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("title");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "title") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("title element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Description {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Description {
    const XML_LOCAL_NAME: &'static str = "description";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("description");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "description") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("description element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub libosinfo_elems: Vec<Libosinfo>,
}
impl Metadata {
    const XML_LOCAL_NAME: &'static str = "metadata";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("metadata");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "metadata") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("metadata element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (Some("http://libosinfo.org/xmlns/libvirt/domain/1.0"), "libosinfo") => {
                        n.libosinfo_elems
                            .push(Libosinfo::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.libosinfo_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Libosinfo {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_metadata_libosinfo_libosinfo_libosinfo_os_elems:
        Vec<DomainMetadataLibosinfoLibosinfoLibosinfoOs>,
}
impl Libosinfo {
    const XML_LOCAL_NAME: &'static str = "libosinfo";
    const XML_NAMESPACE: Option<&'static str> =
        Some("http://libosinfo.org/xmlns/libvirt/domain/1.0");
    const XML_PREFIX: Option<&'static str> = Some("libosinfo");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "libosinfo",
        "http://libosinfo.org/xmlns/libvirt/domain/1.0",
        Some("libosinfo"),
    );
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (Some("http://libosinfo.org/xmlns/libvirt/domain/1.0"), "libosinfo") => {
                        Self::parse_children(attributes, iter)
                    }
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("libosinfo element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (Some("http://libosinfo.org/xmlns/libvirt/domain/1.0"), "os") => {
                        n.domain_metadata_libosinfo_libosinfo_libosinfo_os_elems
                            .push(DomainMetadataLibosinfoLibosinfoLibosinfoOs::parse_children(
                                attributes, iter,
                            )?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self
            .domain_metadata_libosinfo_libosinfo_libosinfo_os_elems
            .iter()
        {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainMetadataLibosinfoLibosinfoLibosinfoOs {
    pub r#id: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainMetadataLibosinfoLibosinfoLibosinfoOs {
    const XML_LOCAL_NAME: &'static str = "os";
    const XML_NAMESPACE: Option<&'static str> =
        Some("http://libosinfo.org/xmlns/libvirt/domain/1.0");
    const XML_PREFIX: Option<&'static str> = Some("libosinfo");
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::qualified(
        "os",
        "http://libosinfo.org/xmlns/libvirt/domain/1.0",
        Some("libosinfo"),
    );
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (Some("http://libosinfo.org/xmlns/libvirt/domain/1.0"), "os") => {
                        Self::parse_children(attributes, iter)
                    }
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("os element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "id") => {
                    n.r#id = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#id.as_ref() {
            el_builder = el_builder.attr("id", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Memory {
    pub r#unit: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Memory {
    const XML_LOCAL_NAME: &'static str = "memory";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("memory");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "memory") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("memory element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "unit") => {
                    n.r#unit = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#unit.as_ref() {
            el_builder = el_builder.attr("unit", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CurrentMemory {
    pub r#unit: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl CurrentMemory {
    const XML_LOCAL_NAME: &'static str = "currentMemory";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("currentMemory");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "currentMemory") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("currentMemory element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "unit") => {
                    n.r#unit = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#unit.as_ref() {
            el_builder = el_builder.attr("unit", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Vcpu {
    pub r#placement: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Vcpu {
    const XML_LOCAL_NAME: &'static str = "vcpu";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("vcpu");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "vcpu") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("vcpu element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "placement") => {
                    n.r#placement = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#placement.as_ref() {
            el_builder = el_builder.attr("placement", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainOs {
    pub r#firmware: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub firmware_elems: Vec<Firmware>,
    pub type_elems: Vec<Type>,
    pub loader_elems: Vec<Loader>,
    pub nvram_elems: Vec<Nvram>,
    pub boot_elems: Vec<Boot>,
    pub smbios_elems: Vec<Smbios>,
}
impl DomainOs {
    const XML_LOCAL_NAME: &'static str = "os";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("os");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "os") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("os element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "firmware") => {
                    n.r#firmware = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "firmware") => {
                        n.firmware_elems
                            .push(Firmware::parse_children(attributes, iter)?);
                    }
                    (None, "type") => {
                        n.type_elems.push(Type::parse_children(attributes, iter)?);
                    }
                    (None, "loader") => {
                        n.loader_elems
                            .push(Loader::parse_children(attributes, iter)?);
                    }
                    (None, "nvram") => {
                        n.nvram_elems.push(Nvram::parse_children(attributes, iter)?);
                    }
                    (None, "boot") => {
                        n.boot_elems.push(Boot::parse_children(attributes, iter)?);
                    }
                    (None, "smbios") => {
                        n.smbios_elems
                            .push(Smbios::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#firmware.as_ref() {
            el_builder = el_builder.attr("firmware", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.firmware_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.type_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.loader_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.nvram_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.boot_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.smbios_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Type {
    pub r#arch: Option<String>,
    pub r#machine: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Type {
    const XML_LOCAL_NAME: &'static str = "type";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("type");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "type") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("type element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "arch") => {
                    n.r#arch = Some(attr.value);
                }
                (None, "machine") => {
                    n.r#machine = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#arch.as_ref() {
            el_builder = el_builder.attr("arch", v);
        }
        if let Some(v) = self.r#machine.as_ref() {
            el_builder = el_builder.attr("machine", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Firmware {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub feature_elems: Vec<Feature>,
}
impl Firmware {
    const XML_LOCAL_NAME: &'static str = "firmware";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("firmware");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "firmware") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("firmware element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "feature") => {
                        n.feature_elems
                            .push(Feature::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.feature_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Feature {
    pub r#enabled: Option<String>,
    pub r#name: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Feature {
    const XML_LOCAL_NAME: &'static str = "feature";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("feature");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "feature") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("feature element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "enabled") => {
                    n.r#enabled = Some(attr.value);
                }
                (None, "name") => {
                    n.r#name = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#enabled.as_ref() {
            el_builder = el_builder.attr("enabled", v);
        }
        if let Some(v) = self.r#name.as_ref() {
            el_builder = el_builder.attr("name", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Loader {
    pub r#readonly: Option<String>,
    pub r#type: Option<String>,
    pub r#format: Option<String>,
    pub r#secure: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Loader {
    const XML_LOCAL_NAME: &'static str = "loader";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("loader");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "loader") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("loader element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "readonly") => {
                    n.r#readonly = Some(attr.value);
                }
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "format") => {
                    n.r#format = Some(attr.value);
                }
                (None, "secure") => {
                    n.r#secure = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#readonly.as_ref() {
            el_builder = el_builder.attr("readonly", v);
        }
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#format.as_ref() {
            el_builder = el_builder.attr("format", v);
        }
        if let Some(v) = self.r#secure.as_ref() {
            el_builder = el_builder.attr("secure", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Nvram {
    pub r#template: Option<String>,
    pub r#template_format: Option<String>,
    pub r#format: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Nvram {
    const XML_LOCAL_NAME: &'static str = "nvram";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("nvram");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "nvram") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("nvram element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "template") => {
                    n.r#template = Some(attr.value);
                }
                (None, "templateFormat") => {
                    n.r#template_format = Some(attr.value);
                }
                (None, "format") => {
                    n.r#format = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#template.as_ref() {
            el_builder = el_builder.attr("template", v);
        }
        if let Some(v) = self.r#template_format.as_ref() {
            el_builder = el_builder.attr("templateFormat", v);
        }
        if let Some(v) = self.r#format.as_ref() {
            el_builder = el_builder.attr("format", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Boot {
    pub r#dev: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Boot {
    const XML_LOCAL_NAME: &'static str = "boot";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("boot");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "boot") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("boot element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "dev") => {
                    n.r#dev = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#dev.as_ref() {
            el_builder = el_builder.attr("dev", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Features {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub acpi_elems: Vec<Acpi>,
    pub apic_elems: Vec<Apic>,
    pub vmport_elems: Vec<Vmport>,
    pub smm_elems: Vec<Smm>,
}
impl Features {
    const XML_LOCAL_NAME: &'static str = "features";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("features");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "features") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("features element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "acpi") => {
                        n.acpi_elems.push(Acpi::parse_children(attributes, iter)?);
                    }
                    (None, "apic") => {
                        n.apic_elems.push(Apic::parse_children(attributes, iter)?);
                    }
                    (None, "vmport") => {
                        n.vmport_elems
                            .push(Vmport::parse_children(attributes, iter)?);
                    }
                    (None, "smm") => {
                        n.smm_elems.push(Smm::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.acpi_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.apic_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.vmport_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.smm_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Acpi {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Acpi {
    const XML_LOCAL_NAME: &'static str = "acpi";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("acpi");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "acpi") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("acpi element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Apic {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Apic {
    const XML_LOCAL_NAME: &'static str = "apic";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("apic");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "apic") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("apic element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Vmport {
    pub r#state: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Vmport {
    const XML_LOCAL_NAME: &'static str = "vmport";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("vmport");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "vmport") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("vmport element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "state") => {
                    n.r#state = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#state.as_ref() {
            el_builder = el_builder.attr("state", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cpu {
    pub r#mode: Option<String>,
    pub r#check: Option<String>,
    pub r#migratable: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub topology_elems: Vec<Topology>,
}
impl Cpu {
    const XML_LOCAL_NAME: &'static str = "cpu";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("cpu");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "cpu") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("cpu element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "mode") => {
                    n.r#mode = Some(attr.value);
                }
                (None, "check") => {
                    n.r#check = Some(attr.value);
                }
                (None, "migratable") => {
                    n.r#migratable = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "topology") => {
                        n.topology_elems
                            .push(Topology::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#mode.as_ref() {
            el_builder = el_builder.attr("mode", v);
        }
        if let Some(v) = self.r#check.as_ref() {
            el_builder = el_builder.attr("check", v);
        }
        if let Some(v) = self.r#migratable.as_ref() {
            el_builder = el_builder.attr("migratable", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.topology_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Topology {
    pub r#sockets: Option<String>,
    pub r#dies: Option<String>,
    pub r#clusters: Option<String>,
    pub r#cores: Option<String>,
    pub r#threads: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Topology {
    const XML_LOCAL_NAME: &'static str = "topology";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("topology");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "topology") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("topology element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "sockets") => {
                    n.r#sockets = Some(attr.value);
                }
                (None, "dies") => {
                    n.r#dies = Some(attr.value);
                }
                (None, "clusters") => {
                    n.r#clusters = Some(attr.value);
                }
                (None, "cores") => {
                    n.r#cores = Some(attr.value);
                }
                (None, "threads") => {
                    n.r#threads = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#sockets.as_ref() {
            el_builder = el_builder.attr("sockets", v);
        }
        if let Some(v) = self.r#dies.as_ref() {
            el_builder = el_builder.attr("dies", v);
        }
        if let Some(v) = self.r#clusters.as_ref() {
            el_builder = el_builder.attr("clusters", v);
        }
        if let Some(v) = self.r#cores.as_ref() {
            el_builder = el_builder.attr("cores", v);
        }
        if let Some(v) = self.r#threads.as_ref() {
            el_builder = el_builder.attr("threads", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Clock {
    pub r#offset: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub timer_elems: Vec<Timer>,
}
impl Clock {
    const XML_LOCAL_NAME: &'static str = "clock";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("clock");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "clock") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("clock element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "offset") => {
                    n.r#offset = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "timer") => {
                        n.timer_elems.push(Timer::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#offset.as_ref() {
            el_builder = el_builder.attr("offset", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.timer_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Timer {
    pub r#name: Option<String>,
    pub r#tickpolicy: Option<String>,
    pub r#present: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Timer {
    const XML_LOCAL_NAME: &'static str = "timer";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("timer");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "timer") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("timer element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "name") => {
                    n.r#name = Some(attr.value);
                }
                (None, "tickpolicy") => {
                    n.r#tickpolicy = Some(attr.value);
                }
                (None, "present") => {
                    n.r#present = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#name.as_ref() {
            el_builder = el_builder.attr("name", v);
        }
        if let Some(v) = self.r#tickpolicy.as_ref() {
            el_builder = el_builder.attr("tickpolicy", v);
        }
        if let Some(v) = self.r#present.as_ref() {
            el_builder = el_builder.attr("present", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OnPoweroff {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl OnPoweroff {
    const XML_LOCAL_NAME: &'static str = "on_poweroff";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("on_poweroff");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "on_poweroff") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("on_poweroff element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OnReboot {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl OnReboot {
    const XML_LOCAL_NAME: &'static str = "on_reboot";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("on_reboot");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "on_reboot") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("on_reboot element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OnCrash {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl OnCrash {
    const XML_LOCAL_NAME: &'static str = "on_crash";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("on_crash");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "on_crash") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("on_crash element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pm {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub suspend_to_disk_elems: Vec<SuspendToDisk>,
    pub suspend_to_mem_elems: Vec<SuspendToMem>,
}
impl Pm {
    const XML_LOCAL_NAME: &'static str = "pm";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("pm");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "pm") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("pm element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "suspend-to-disk") => {
                        n.suspend_to_disk_elems
                            .push(SuspendToDisk::parse_children(attributes, iter)?);
                    }
                    (None, "suspend-to-mem") => {
                        n.suspend_to_mem_elems
                            .push(SuspendToMem::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.suspend_to_disk_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.suspend_to_mem_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SuspendToMem {
    pub r#enabled: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl SuspendToMem {
    const XML_LOCAL_NAME: &'static str = "suspend-to-mem";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("suspend-to-mem");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "suspend-to-mem") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("suspend-to-mem element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "enabled") => {
                    n.r#enabled = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#enabled.as_ref() {
            el_builder = el_builder.attr("enabled", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SuspendToDisk {
    pub r#enabled: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl SuspendToDisk {
    const XML_LOCAL_NAME: &'static str = "suspend-to-disk";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("suspend-to-disk");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "suspend-to-disk") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("suspend-to-disk element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "enabled") => {
                    n.r#enabled = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#enabled.as_ref() {
            el_builder = el_builder.attr("enabled", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Devices {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub controller_elems: Vec<Controller>,
    pub interface_elems: Vec<Interface>,
    pub serial_elems: Vec<Serial>,
    pub channel_elems: Vec<Channel>,
    pub sound_elems: Vec<Sound>,
    pub audio_elems: Vec<Audio>,
    pub redirdev_elems: Vec<Redirdev>,
    pub watchdog_elems: Vec<Watchdog>,
    pub memballoon_elems: Vec<Memballoon>,
    pub rng_elems: Vec<Rng>,
    pub input_elems: Vec<Input>,
    pub emulator_elems: Vec<Emulator>,
    pub disk_elems: Vec<Disk>,
    pub graphics_elems: Vec<Graphics>,
    pub video_elems: Vec<Video>,
    pub console_elems: Vec<Console>,
}
impl Devices {
    const XML_LOCAL_NAME: &'static str = "devices";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("devices");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "devices") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("devices element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "controller") => {
                        n.controller_elems
                            .push(Controller::parse_children(attributes, iter)?);
                    }
                    (None, "interface") => {
                        n.interface_elems
                            .push(Interface::parse_children(attributes, iter)?);
                    }
                    (None, "serial") => {
                        n.serial_elems
                            .push(Serial::parse_children(attributes, iter)?);
                    }
                    (None, "channel") => {
                        n.channel_elems
                            .push(Channel::parse_children(attributes, iter)?);
                    }
                    (None, "sound") => {
                        n.sound_elems.push(Sound::parse_children(attributes, iter)?);
                    }
                    (None, "audio") => {
                        n.audio_elems.push(Audio::parse_children(attributes, iter)?);
                    }
                    (None, "redirdev") => {
                        n.redirdev_elems
                            .push(Redirdev::parse_children(attributes, iter)?);
                    }
                    (None, "watchdog") => {
                        n.watchdog_elems
                            .push(Watchdog::parse_children(attributes, iter)?);
                    }
                    (None, "memballoon") => {
                        n.memballoon_elems
                            .push(Memballoon::parse_children(attributes, iter)?);
                    }
                    (None, "rng") => {
                        n.rng_elems.push(Rng::parse_children(attributes, iter)?);
                    }
                    (None, "input") => {
                        n.input_elems.push(Input::parse_children(attributes, iter)?);
                    }
                    (None, "emulator") => {
                        n.emulator_elems
                            .push(Emulator::parse_children(attributes, iter)?);
                    }
                    (None, "disk") => {
                        n.disk_elems.push(Disk::parse_children(attributes, iter)?);
                    }
                    (None, "graphics") => {
                        n.graphics_elems
                            .push(Graphics::parse_children(attributes, iter)?);
                    }
                    (None, "video") => {
                        n.video_elems.push(Video::parse_children(attributes, iter)?);
                    }
                    (None, "console") => {
                        n.console_elems
                            .push(Console::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.controller_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.interface_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.serial_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.channel_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.sound_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.audio_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.redirdev_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.watchdog_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.memballoon_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.rng_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.input_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.emulator_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.disk_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.graphics_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.video_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.console_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Emulator {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Emulator {
    const XML_LOCAL_NAME: &'static str = "emulator";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("emulator");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "emulator") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("emulator element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Disk {
    pub r#type: Option<String>,
    pub r#device: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub driver_elems: Vec<Driver>,
    pub domain_devices_disk_source_elems: Vec<DomainDevicesDiskSource>,
    pub backing_store_elems: Vec<BackingStore>,
    pub domain_devices_disk_target_elems: Vec<DomainDevicesDiskTarget>,
    pub readonly_elems: Vec<Readonly>,
    pub domain_devices_disk_alias_elems: Vec<DomainDevicesDiskAlias>,
    pub domain_devices_disk_address_elems: Vec<DomainDevicesDiskAddress>,
}
impl Disk {
    const XML_LOCAL_NAME: &'static str = "disk";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("disk");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "disk") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("disk element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "device") => {
                    n.r#device = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "driver") => {
                        n.driver_elems
                            .push(Driver::parse_children(attributes, iter)?);
                    }
                    (None, "source") => {
                        n.domain_devices_disk_source_elems
                            .push(DomainDevicesDiskSource::parse_children(attributes, iter)?);
                    }
                    (None, "backingStore") => {
                        n.backing_store_elems
                            .push(BackingStore::parse_children(attributes, iter)?);
                    }
                    (None, "target") => {
                        n.domain_devices_disk_target_elems
                            .push(DomainDevicesDiskTarget::parse_children(attributes, iter)?);
                    }
                    (None, "readonly") => {
                        n.readonly_elems
                            .push(Readonly::parse_children(attributes, iter)?);
                    }
                    (None, "alias") => {
                        n.domain_devices_disk_alias_elems
                            .push(DomainDevicesDiskAlias::parse_children(attributes, iter)?);
                    }
                    (None, "address") => {
                        n.domain_devices_disk_address_elems
                            .push(DomainDevicesDiskAddress::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#device.as_ref() {
            el_builder = el_builder.attr("device", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.driver_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_disk_source_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.backing_store_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_disk_target_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.readonly_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_disk_alias_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_disk_address_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Driver {
    pub r#name: Option<String>,
    pub r#type: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Driver {
    const XML_LOCAL_NAME: &'static str = "driver";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("driver");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "driver") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("driver element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "name") => {
                    n.r#name = Some(attr.value);
                }
                (None, "type") => {
                    n.r#type = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#name.as_ref() {
            el_builder = el_builder.attr("name", v);
        }
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesDiskSource {
    pub r#file: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesDiskSource {
    const XML_LOCAL_NAME: &'static str = "source";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("source");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "source") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("source element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "file") => {
                    n.r#file = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#file.as_ref() {
            el_builder = el_builder.attr("file", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesDiskTarget {
    pub r#dev: Option<String>,
    pub r#bus: Option<String>,
    pub r#removable: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesDiskTarget {
    const XML_LOCAL_NAME: &'static str = "target";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("target");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "target") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("target element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "dev") => {
                    n.r#dev = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "removable") => {
                    n.r#removable = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#dev.as_ref() {
            el_builder = el_builder.attr("dev", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#removable.as_ref() {
            el_builder = el_builder.attr("removable", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesDiskAddress {
    pub r#type: Option<String>,
    pub r#domain: Option<String>,
    pub r#bus: Option<String>,
    pub r#slot: Option<String>,
    pub r#function: Option<String>,
    pub r#controller: Option<String>,
    pub r#target: Option<String>,
    pub r#unit: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesDiskAddress {
    const XML_LOCAL_NAME: &'static str = "address";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("address");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("address element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "domain") => {
                    n.r#domain = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "slot") => {
                    n.r#slot = Some(attr.value);
                }
                (None, "function") => {
                    n.r#function = Some(attr.value);
                }
                (None, "controller") => {
                    n.r#controller = Some(attr.value);
                }
                (None, "target") => {
                    n.r#target = Some(attr.value);
                }
                (None, "unit") => {
                    n.r#unit = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#domain.as_ref() {
            el_builder = el_builder.attr("domain", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#slot.as_ref() {
            el_builder = el_builder.attr("slot", v);
        }
        if let Some(v) = self.r#function.as_ref() {
            el_builder = el_builder.attr("function", v);
        }
        if let Some(v) = self.r#controller.as_ref() {
            el_builder = el_builder.attr("controller", v);
        }
        if let Some(v) = self.r#target.as_ref() {
            el_builder = el_builder.attr("target", v);
        }
        if let Some(v) = self.r#unit.as_ref() {
            el_builder = el_builder.attr("unit", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Readonly {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Readonly {
    const XML_LOCAL_NAME: &'static str = "readonly";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("readonly");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "readonly") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("readonly element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Controller {
    pub r#type: Option<String>,
    pub r#index: Option<String>,
    pub r#model: Option<String>,
    pub r#ports: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_devices_controller_model_elems: Vec<DomainDevicesControllerModel>,
    pub domain_devices_controller_target_elems: Vec<DomainDevicesControllerTarget>,
    pub domain_devices_controller_address_elems: Vec<DomainDevicesControllerAddress>,
}
impl Controller {
    const XML_LOCAL_NAME: &'static str = "controller";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("controller");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "controller") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("controller element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "index") => {
                    n.r#index = Some(attr.value);
                }
                (None, "model") => {
                    n.r#model = Some(attr.value);
                }
                (None, "ports") => {
                    n.r#ports = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "model") => {
                        n.domain_devices_controller_model_elems.push(
                            DomainDevicesControllerModel::parse_children(attributes, iter)?,
                        );
                    }
                    (None, "target") => {
                        n.domain_devices_controller_target_elems.push(
                            DomainDevicesControllerTarget::parse_children(attributes, iter)?,
                        );
                    }
                    (None, "address") => {
                        n.domain_devices_controller_address_elems.push(
                            DomainDevicesControllerAddress::parse_children(attributes, iter)?,
                        );
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#index.as_ref() {
            el_builder = el_builder.attr("index", v);
        }
        if let Some(v) = self.r#model.as_ref() {
            el_builder = el_builder.attr("model", v);
        }
        if let Some(v) = self.r#ports.as_ref() {
            el_builder = el_builder.attr("ports", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.domain_devices_controller_model_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_controller_target_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_controller_address_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesControllerAddress {
    pub r#type: Option<String>,
    pub r#domain: Option<String>,
    pub r#bus: Option<String>,
    pub r#slot: Option<String>,
    pub r#function: Option<String>,
    pub r#multifunction: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesControllerAddress {
    const XML_LOCAL_NAME: &'static str = "address";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("address");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("address element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "domain") => {
                    n.r#domain = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "slot") => {
                    n.r#slot = Some(attr.value);
                }
                (None, "function") => {
                    n.r#function = Some(attr.value);
                }
                (None, "multifunction") => {
                    n.r#multifunction = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#domain.as_ref() {
            el_builder = el_builder.attr("domain", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#slot.as_ref() {
            el_builder = el_builder.attr("slot", v);
        }
        if let Some(v) = self.r#function.as_ref() {
            el_builder = el_builder.attr("function", v);
        }
        if let Some(v) = self.r#multifunction.as_ref() {
            el_builder = el_builder.attr("multifunction", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesControllerModel {
    pub r#name: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesControllerModel {
    const XML_LOCAL_NAME: &'static str = "model";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("model");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "model") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("model element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "name") => {
                    n.r#name = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#name.as_ref() {
            el_builder = el_builder.attr("name", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesControllerTarget {
    pub r#chassis: Option<String>,
    pub r#port: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesControllerTarget {
    const XML_LOCAL_NAME: &'static str = "target";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("target");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "target") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("target element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "chassis") => {
                    n.r#chassis = Some(attr.value);
                }
                (None, "port") => {
                    n.r#port = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#chassis.as_ref() {
            el_builder = el_builder.attr("chassis", v);
        }
        if let Some(v) = self.r#port.as_ref() {
            el_builder = el_builder.attr("port", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Interface {
    pub r#type: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub mac_elems: Vec<Mac>,
    pub domain_devices_interface_source_elems: Vec<DomainDevicesInterfaceSource>,
    pub domain_devices_interface_model_elems: Vec<DomainDevicesInterfaceModel>,
    pub domain_devices_interface_address_elems: Vec<DomainDevicesInterfaceAddress>,
}
impl Interface {
    const XML_LOCAL_NAME: &'static str = "interface";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("interface");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "interface") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("interface element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "mac") => {
                        n.mac_elems.push(Mac::parse_children(attributes, iter)?);
                    }
                    (None, "source") => {
                        n.domain_devices_interface_source_elems.push(
                            DomainDevicesInterfaceSource::parse_children(attributes, iter)?,
                        );
                    }
                    (None, "model") => {
                        n.domain_devices_interface_model_elems.push(
                            DomainDevicesInterfaceModel::parse_children(attributes, iter)?,
                        );
                    }
                    (None, "address") => {
                        n.domain_devices_interface_address_elems.push(
                            DomainDevicesInterfaceAddress::parse_children(attributes, iter)?,
                        );
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.mac_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_interface_source_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_interface_model_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_interface_address_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Mac {
    pub r#address: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Mac {
    const XML_LOCAL_NAME: &'static str = "mac";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("mac");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "mac") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("mac element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "address") => {
                    n.r#address = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#address.as_ref() {
            el_builder = el_builder.attr("address", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesInterfaceSource {
    pub r#network: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesInterfaceSource {
    const XML_LOCAL_NAME: &'static str = "source";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("source");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "source") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("source element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "network") => {
                    n.r#network = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#network.as_ref() {
            el_builder = el_builder.attr("network", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesInterfaceModel {
    pub r#type: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesInterfaceModel {
    const XML_LOCAL_NAME: &'static str = "model";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("model");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "model") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("model element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesInterfaceAddress {
    pub r#type: Option<String>,
    pub r#domain: Option<String>,
    pub r#bus: Option<String>,
    pub r#slot: Option<String>,
    pub r#function: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesInterfaceAddress {
    const XML_LOCAL_NAME: &'static str = "address";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("address");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("address element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "domain") => {
                    n.r#domain = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "slot") => {
                    n.r#slot = Some(attr.value);
                }
                (None, "function") => {
                    n.r#function = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#domain.as_ref() {
            el_builder = el_builder.attr("domain", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#slot.as_ref() {
            el_builder = el_builder.attr("slot", v);
        }
        if let Some(v) = self.r#function.as_ref() {
            el_builder = el_builder.attr("function", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Serial {
    pub r#type: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_devices_serial_target_elems: Vec<DomainDevicesSerialTarget>,
}
impl Serial {
    const XML_LOCAL_NAME: &'static str = "serial";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("serial");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "serial") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("serial element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "target") => {
                        n.domain_devices_serial_target_elems
                            .push(DomainDevicesSerialTarget::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.domain_devices_serial_target_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesSerialTarget {
    pub r#type: Option<String>,
    pub r#port: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_devices_serial_target_model_elems: Vec<DomainDevicesSerialTargetModel>,
}
impl DomainDevicesSerialTarget {
    const XML_LOCAL_NAME: &'static str = "target";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("target");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "target") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("target element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "port") => {
                    n.r#port = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "model") => {
                        n.domain_devices_serial_target_model_elems.push(
                            DomainDevicesSerialTargetModel::parse_children(attributes, iter)?,
                        );
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#port.as_ref() {
            el_builder = el_builder.attr("port", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.domain_devices_serial_target_model_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesSerialTargetModel {
    pub r#name: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesSerialTargetModel {
    const XML_LOCAL_NAME: &'static str = "model";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("model");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "model") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("model element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "name") => {
                    n.r#name = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#name.as_ref() {
            el_builder = el_builder.attr("name", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Console {
    pub r#type: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_devices_console_target_elems: Vec<DomainDevicesConsoleTarget>,
}
impl Console {
    const XML_LOCAL_NAME: &'static str = "console";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("console");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "console") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("console element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "target") => {
                        n.domain_devices_console_target_elems.push(
                            DomainDevicesConsoleTarget::parse_children(attributes, iter)?,
                        );
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.domain_devices_console_target_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesConsoleTarget {
    pub r#type: Option<String>,
    pub r#port: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesConsoleTarget {
    const XML_LOCAL_NAME: &'static str = "target";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("target");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "target") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("target element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "port") => {
                    n.r#port = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#port.as_ref() {
            el_builder = el_builder.attr("port", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Channel {
    pub r#type: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_devices_channel_target_elems: Vec<DomainDevicesChannelTarget>,
    pub domain_devices_channel_address_elems: Vec<DomainDevicesChannelAddress>,
}
impl Channel {
    const XML_LOCAL_NAME: &'static str = "channel";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("channel");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "channel") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("channel element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "target") => {
                        n.domain_devices_channel_target_elems.push(
                            DomainDevicesChannelTarget::parse_children(attributes, iter)?,
                        );
                    }
                    (None, "address") => {
                        n.domain_devices_channel_address_elems.push(
                            DomainDevicesChannelAddress::parse_children(attributes, iter)?,
                        );
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.domain_devices_channel_target_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_channel_address_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesChannelTarget {
    pub r#type: Option<String>,
    pub r#name: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesChannelTarget {
    const XML_LOCAL_NAME: &'static str = "target";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("target");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "target") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("target element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "name") => {
                    n.r#name = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#name.as_ref() {
            el_builder = el_builder.attr("name", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesChannelAddress {
    pub r#type: Option<String>,
    pub r#controller: Option<String>,
    pub r#bus: Option<String>,
    pub r#port: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesChannelAddress {
    const XML_LOCAL_NAME: &'static str = "address";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("address");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("address element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "controller") => {
                    n.r#controller = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "port") => {
                    n.r#port = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#controller.as_ref() {
            el_builder = el_builder.attr("controller", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#port.as_ref() {
            el_builder = el_builder.attr("port", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Input {
    pub r#type: Option<String>,
    pub r#bus: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_devices_input_address_elems: Vec<DomainDevicesInputAddress>,
}
impl Input {
    const XML_LOCAL_NAME: &'static str = "input";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("input");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "input") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("input element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => {
                        n.domain_devices_input_address_elems
                            .push(DomainDevicesInputAddress::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.domain_devices_input_address_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesInputAddress {
    pub r#type: Option<String>,
    pub r#bus: Option<String>,
    pub r#port: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesInputAddress {
    const XML_LOCAL_NAME: &'static str = "address";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("address");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("address element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "port") => {
                    n.r#port = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#port.as_ref() {
            el_builder = el_builder.attr("port", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Graphics {
    pub r#type: Option<String>,
    pub r#autoport: Option<String>,
    pub r#listen: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub image_elems: Vec<Image>,
    pub listen_elems: Vec<Listen>,
}
impl Graphics {
    const XML_LOCAL_NAME: &'static str = "graphics";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("graphics");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "graphics") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("graphics element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "autoport") => {
                    n.r#autoport = Some(attr.value);
                }
                (None, "listen") => {
                    n.r#listen = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "image") => {
                        n.image_elems.push(Image::parse_children(attributes, iter)?);
                    }
                    (None, "listen") => {
                        n.listen_elems
                            .push(Listen::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#autoport.as_ref() {
            el_builder = el_builder.attr("autoport", v);
        }
        if let Some(v) = self.r#listen.as_ref() {
            el_builder = el_builder.attr("listen", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.image_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.listen_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Listen {
    pub r#type: Option<String>,
    pub r#address: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Listen {
    const XML_LOCAL_NAME: &'static str = "listen";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("listen");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "listen") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("listen element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "address") => {
                    n.r#address = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#address.as_ref() {
            el_builder = el_builder.attr("address", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Image {
    pub r#compression: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Image {
    const XML_LOCAL_NAME: &'static str = "image";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("image");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "image") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("image element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "compression") => {
                    n.r#compression = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#compression.as_ref() {
            el_builder = el_builder.attr("compression", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Sound {
    pub r#model: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_devices_sound_address_elems: Vec<DomainDevicesSoundAddress>,
}
impl Sound {
    const XML_LOCAL_NAME: &'static str = "sound";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("sound");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "sound") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("sound element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "model") => {
                    n.r#model = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => {
                        n.domain_devices_sound_address_elems
                            .push(DomainDevicesSoundAddress::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#model.as_ref() {
            el_builder = el_builder.attr("model", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.domain_devices_sound_address_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesSoundAddress {
    pub r#type: Option<String>,
    pub r#domain: Option<String>,
    pub r#bus: Option<String>,
    pub r#slot: Option<String>,
    pub r#function: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesSoundAddress {
    const XML_LOCAL_NAME: &'static str = "address";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("address");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("address element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "domain") => {
                    n.r#domain = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "slot") => {
                    n.r#slot = Some(attr.value);
                }
                (None, "function") => {
                    n.r#function = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#domain.as_ref() {
            el_builder = el_builder.attr("domain", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#slot.as_ref() {
            el_builder = el_builder.attr("slot", v);
        }
        if let Some(v) = self.r#function.as_ref() {
            el_builder = el_builder.attr("function", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Audio {
    pub r#id: Option<String>,
    pub r#type: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Audio {
    const XML_LOCAL_NAME: &'static str = "audio";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("audio");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "audio") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("audio element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "id") => {
                    n.r#id = Some(attr.value);
                }
                (None, "type") => {
                    n.r#type = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#id.as_ref() {
            el_builder = el_builder.attr("id", v);
        }
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Video {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_devices_video_model_elems: Vec<DomainDevicesVideoModel>,
    pub domain_devices_video_alias_elems: Vec<DomainDevicesVideoAlias>,
    pub domain_devices_video_address_elems: Vec<DomainDevicesVideoAddress>,
}
impl Video {
    const XML_LOCAL_NAME: &'static str = "video";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("video");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "video") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("video element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "model") => {
                        n.domain_devices_video_model_elems
                            .push(DomainDevicesVideoModel::parse_children(attributes, iter)?);
                    }
                    (None, "alias") => {
                        n.domain_devices_video_alias_elems
                            .push(DomainDevicesVideoAlias::parse_children(attributes, iter)?);
                    }
                    (None, "address") => {
                        n.domain_devices_video_address_elems
                            .push(DomainDevicesVideoAddress::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.domain_devices_video_model_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_video_alias_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_video_address_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesVideoModel {
    pub r#type: Option<String>,
    pub r#heads: Option<String>,
    pub r#primary: Option<String>,
    pub r#ram: Option<String>,
    pub r#vram: Option<String>,
    pub r#vgamem: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub resolution_elems: Vec<Resolution>,
}
impl DomainDevicesVideoModel {
    const XML_LOCAL_NAME: &'static str = "model";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("model");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "model") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("model element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "heads") => {
                    n.r#heads = Some(attr.value);
                }
                (None, "primary") => {
                    n.r#primary = Some(attr.value);
                }
                (None, "ram") => {
                    n.r#ram = Some(attr.value);
                }
                (None, "vram") => {
                    n.r#vram = Some(attr.value);
                }
                (None, "vgamem") => {
                    n.r#vgamem = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "resolution") => {
                        n.resolution_elems
                            .push(Resolution::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#heads.as_ref() {
            el_builder = el_builder.attr("heads", v);
        }
        if let Some(v) = self.r#primary.as_ref() {
            el_builder = el_builder.attr("primary", v);
        }
        if let Some(v) = self.r#ram.as_ref() {
            el_builder = el_builder.attr("ram", v);
        }
        if let Some(v) = self.r#vram.as_ref() {
            el_builder = el_builder.attr("vram", v);
        }
        if let Some(v) = self.r#vgamem.as_ref() {
            el_builder = el_builder.attr("vgamem", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.resolution_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesVideoAddress {
    pub r#type: Option<String>,
    pub r#domain: Option<String>,
    pub r#bus: Option<String>,
    pub r#slot: Option<String>,
    pub r#function: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesVideoAddress {
    const XML_LOCAL_NAME: &'static str = "address";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("address");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("address element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "domain") => {
                    n.r#domain = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "slot") => {
                    n.r#slot = Some(attr.value);
                }
                (None, "function") => {
                    n.r#function = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#domain.as_ref() {
            el_builder = el_builder.attr("domain", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#slot.as_ref() {
            el_builder = el_builder.attr("slot", v);
        }
        if let Some(v) = self.r#function.as_ref() {
            el_builder = el_builder.attr("function", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Redirdev {
    pub r#bus: Option<String>,
    pub r#type: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_devices_redirdev_address_elems: Vec<DomainDevicesRedirdevAddress>,
}
impl Redirdev {
    const XML_LOCAL_NAME: &'static str = "redirdev";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("redirdev");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "redirdev") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("redirdev element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "type") => {
                    n.r#type = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => {
                        n.domain_devices_redirdev_address_elems.push(
                            DomainDevicesRedirdevAddress::parse_children(attributes, iter)?,
                        );
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.domain_devices_redirdev_address_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesRedirdevAddress {
    pub r#type: Option<String>,
    pub r#bus: Option<String>,
    pub r#port: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesRedirdevAddress {
    const XML_LOCAL_NAME: &'static str = "address";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("address");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("address element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "port") => {
                    n.r#port = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#port.as_ref() {
            el_builder = el_builder.attr("port", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Watchdog {
    pub r#model: Option<String>,
    pub r#action: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Watchdog {
    const XML_LOCAL_NAME: &'static str = "watchdog";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("watchdog");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "watchdog") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("watchdog element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "model") => {
                    n.r#model = Some(attr.value);
                }
                (None, "action") => {
                    n.r#action = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#model.as_ref() {
            el_builder = el_builder.attr("model", v);
        }
        if let Some(v) = self.r#action.as_ref() {
            el_builder = el_builder.attr("action", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Memballoon {
    pub r#model: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub domain_devices_memballoon_address_elems: Vec<DomainDevicesMemballoonAddress>,
}
impl Memballoon {
    const XML_LOCAL_NAME: &'static str = "memballoon";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("memballoon");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "memballoon") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("memballoon element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "model") => {
                    n.r#model = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => {
                        n.domain_devices_memballoon_address_elems.push(
                            DomainDevicesMemballoonAddress::parse_children(attributes, iter)?,
                        );
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#model.as_ref() {
            el_builder = el_builder.attr("model", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.domain_devices_memballoon_address_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesMemballoonAddress {
    pub r#type: Option<String>,
    pub r#domain: Option<String>,
    pub r#bus: Option<String>,
    pub r#slot: Option<String>,
    pub r#function: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesMemballoonAddress {
    const XML_LOCAL_NAME: &'static str = "address";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("address");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("address element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "domain") => {
                    n.r#domain = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "slot") => {
                    n.r#slot = Some(attr.value);
                }
                (None, "function") => {
                    n.r#function = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#domain.as_ref() {
            el_builder = el_builder.attr("domain", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#slot.as_ref() {
            el_builder = el_builder.attr("slot", v);
        }
        if let Some(v) = self.r#function.as_ref() {
            el_builder = el_builder.attr("function", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Rng {
    pub r#model: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub backend_elems: Vec<Backend>,
    pub domain_devices_rng_address_elems: Vec<DomainDevicesRngAddress>,
}
impl Rng {
    const XML_LOCAL_NAME: &'static str = "rng";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("rng");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "rng") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("rng element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "model") => {
                    n.r#model = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "backend") => {
                        n.backend_elems
                            .push(Backend::parse_children(attributes, iter)?);
                    }
                    (None, "address") => {
                        n.domain_devices_rng_address_elems
                            .push(DomainDevicesRngAddress::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#model.as_ref() {
            el_builder = el_builder.attr("model", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.backend_elems.iter() {
            child.write_element(w, false)?;
        }
        for child in self.domain_devices_rng_address_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Backend {
    pub r#model: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Backend {
    const XML_LOCAL_NAME: &'static str = "backend";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("backend");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "backend") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("backend element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "model") => {
                    n.r#model = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#model.as_ref() {
            el_builder = el_builder.attr("model", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesRngAddress {
    pub r#type: Option<String>,
    pub r#domain: Option<String>,
    pub r#bus: Option<String>,
    pub r#slot: Option<String>,
    pub r#function: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesRngAddress {
    const XML_LOCAL_NAME: &'static str = "address";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("address");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "address") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("address element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
                }
                (None, "domain") => {
                    n.r#domain = Some(attr.value);
                }
                (None, "bus") => {
                    n.r#bus = Some(attr.value);
                }
                (None, "slot") => {
                    n.r#slot = Some(attr.value);
                }
                (None, "function") => {
                    n.r#function = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        if let Some(v) = self.r#domain.as_ref() {
            el_builder = el_builder.attr("domain", v);
        }
        if let Some(v) = self.r#bus.as_ref() {
            el_builder = el_builder.attr("bus", v);
        }
        if let Some(v) = self.r#slot.as_ref() {
            el_builder = el_builder.attr("slot", v);
        }
        if let Some(v) = self.r#function.as_ref() {
            el_builder = el_builder.attr("function", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Smbios {
    pub r#mode: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Smbios {
    const XML_LOCAL_NAME: &'static str = "smbios";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("smbios");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "smbios") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("smbios element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "mode") => {
                    n.r#mode = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#mode.as_ref() {
            el_builder = el_builder.attr("mode", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Sysinfo {
    pub r#type: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub system_elems: Vec<System>,
}
impl Sysinfo {
    const XML_LOCAL_NAME: &'static str = "sysinfo";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("sysinfo");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "sysinfo") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("sysinfo element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "type") => {
                    n.r#type = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "system") => {
                        n.system_elems
                            .push(System::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#type.as_ref() {
            el_builder = el_builder.attr("type", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.system_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct System {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub entry_elems: Vec<Entry>,
}
impl System {
    const XML_LOCAL_NAME: &'static str = "system";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("system");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "system") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("system element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "entry") => {
                        n.entry_elems.push(Entry::parse_children(attributes, iter)?);
                    }
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for child in self.entry_elems.iter() {
            child.write_element(w, false)?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    pub r#name: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
    pub value: Option<String>,
}
impl Entry {
    const XML_LOCAL_NAME: &'static str = "entry";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("entry");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "entry") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("entry element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "name") => {
                    n.r#name = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
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
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#name.as_ref() {
            el_builder = el_builder.attr("name", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        if let Some(val) = self.value.as_deref() {
            w.write(xml::writer::XmlEvent::characters(val))?;
        }
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Smm {
    pub r#state: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Smm {
    const XML_LOCAL_NAME: &'static str = "smm";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("smm");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "smm") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("smm element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "state") => {
                    n.r#state = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#state.as_ref() {
            el_builder = el_builder.attr("state", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BackingStore {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl BackingStore {
    const XML_LOCAL_NAME: &'static str = "backingStore";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("backingStore");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "backingStore") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("backingStore element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesDiskAlias {
    pub r#name: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesDiskAlias {
    const XML_LOCAL_NAME: &'static str = "alias";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("alias");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "alias") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("alias element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "name") => {
                    n.r#name = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#name.as_ref() {
            el_builder = el_builder.attr("name", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Resolution {
    pub r#x: Option<String>,
    pub r#y: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl Resolution {
    const XML_LOCAL_NAME: &'static str = "resolution";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("resolution");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "resolution") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("resolution element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "x") => {
                    n.r#x = Some(attr.value);
                }
                (None, "y") => {
                    n.r#y = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#x.as_ref() {
            el_builder = el_builder.attr("x", v);
        }
        if let Some(v) = self.r#y.as_ref() {
            el_builder = el_builder.attr("y", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DomainDevicesVideoAlias {
    pub r#name: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    #[serde(skip)]
    pub misc_content: Vec<xml::reader::XmlEvent>,
}
impl DomainDevicesVideoAlias {
    const XML_LOCAL_NAME: &'static str = "alias";
    const XML_NAMESPACE: Option<&'static str> = None;
    const XML_PREFIX: Option<&'static str> = None;
    const XML_RS_NAME: xml::name::Name<'static> = xml::name::Name::local("alias");
    pub fn parse_element<R: std::io::Read>(
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        while let Some(event) = iter.next() {
            return match event {
                Ok(xml::reader::XmlEvent::StartDocument { .. }) => {
                    continue;
                }
                Ok(xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    (None, "alias") => Self::parse_children(attributes, iter),
                    _ => Err(XmlParseError::UnexpectedElement(name)),
                },
                Ok(e) => Err(XmlParseError::UnexpectedXmlEvent(e)),
                Err(e) => Err(e.into()),
            };
        }
        Err(XmlParseError::UnexpectedEof("alias element"))
    }
    fn parse_children<R: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<R>,
    ) -> Result<Self, XmlParseError> {
        let mut n = Self::default();
        for attr in attrs.into_iter() {
            match (
                attr.name.namespace.as_deref(),
                attr.name.local_name.as_str(),
            ) {
                (None, "name") => {
                    n.r#name = Some(attr.value);
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
                    name,
                    attributes,
                    namespace,
                }) => match (name.namespace.as_deref(), name.local_name.as_str()) {
                    _ => {
                        let mut depth: usize = 1;
                        n.misc_content.push(xml::reader::XmlEvent::StartElement {
                            name,
                            attributes,
                            namespace,
                        });
                        while let Some(e) = iter.next() {
                            match e {
                                Ok(xml::reader::XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                }) => {
                                    n.misc_content.push(xml::reader::XmlEvent::StartElement {
                                        name,
                                        attributes,
                                        namespace,
                                    });
                                    depth += 1;
                                }
                                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                                    n.misc_content
                                        .push(xml::reader::XmlEvent::EndElement { name });
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
                        }
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return Ok(n);
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    return Err(XmlParseError::UnexpectedCharacters(
                        XmlDocumentReference::Unknown,
                    ));
                }
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        return Err(XmlParseError::ExpectedEndElement(
            XmlDocumentReference::Unknown,
        ));
    }
    pub fn write_element<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
        include_ns: bool,
    ) -> Result<(), XmlWriteError> {
        let mut el_builder = xml::writer::XmlEvent::start_element(Self::XML_RS_NAME);
        if let Some(v) = self.r#name.as_ref() {
            el_builder = el_builder.attr("name", v);
        }
        for ((_ns, attr_local_name), v) in &self.misc_attrs {
            el_builder = el_builder.attr(attr_local_name.as_str(), &v);
        }
        if include_ns {
            for (k, v) in DOCUMENT_NAMESPACES {
                el_builder = el_builder.ns(*k, *v);
            }
        }
        w.write(el_builder)?;
        for elem in self.misc_content.iter() {
            if let Some(writer_event) = elem.as_writer_event() {
                w.write(writer_event)?;
            }
        }
        w.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}
