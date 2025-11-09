use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssDocument {
    pub r#version: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub channel_elems: Vec<Channel>,
}
impl RssDocument {
    pub fn parse_document<R: std::io::Read>(mut reader: R) -> Self {
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
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                            .push(Channel::parse_children(attributes, iter));
                    }
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {}
                _ => {}
            }
        }
        n
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
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                            .push(RssChannelTitle::parse_children(attributes, iter));
                    }
                    (None, "link") => {
                        n.rss_channel_link_elems
                            .push(RssChannelLink::parse_children(attributes, iter));
                    }
                    (None, "language") => {
                        n.language_elems
                            .push(Language::parse_children(attributes, iter));
                    }
                    (None, "copyright") => {
                        n.copyright_elems
                            .push(Copyright::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "author") => {
                        n.author_elems
                            .push(Author::parse_children(attributes, iter));
                    }
                    (None, "description") => {
                        n.rss_channel_description_elems
                            .push(RssChannelDescription::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "type") => {
                        n.type_elems.push(Type::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "image") => {
                        n.rss_channel_itunes_image_elems
                            .push(RssChannelItunesImage::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "category") => {
                        n.rss_channel_itunes_category_elems
                            .push(RssChannelItunesCategory::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "explicit") => {
                        n.rss_channel_itunes_explicit_elems
                            .push(RssChannelItunesExplicit::parse_children(attributes, iter));
                    }
                    (None, "item") => {
                        n.item_elems.push(Item::parse_children(attributes, iter));
                    }
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {}
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelTitle {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelTitle {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelLink {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelLink {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Language {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Language {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Copyright {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Copyright {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Author {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelDescription {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelDescription {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Type {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Type {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesImage {
    pub r#href: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
}
impl RssChannelItunesImage {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {}
                _ => {}
            }
        }
        n
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
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                            ),
                        );
                    }
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {}
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesCategoryItunesCategory {
    pub r#text: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
}
impl RssChannelItunesCategoryItunesCategory {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {}
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesExplicit {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItunesExplicit {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
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
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                            .push(RssChannelItemItunesTitle::parse_children(attributes, iter));
                    }
                    (None, "link") => {
                        n.rss_channel_item_link_elems
                            .push(RssChannelItemLink::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "image") => {
                        n.rss_channel_item_itunes_image_elems
                            .push(RssChannelItemItunesImage::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "episodeType") => {
                        n.episode_type_elems
                            .push(EpisodeType::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "episode") => {
                        n.episode_elems
                            .push(Episode::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "season") => {
                        n.season_elems
                            .push(Season::parse_children(attributes, iter));
                    }
                    (None, "title") => {
                        n.rss_channel_item_title_elems
                            .push(RssChannelItemTitle::parse_children(attributes, iter));
                    }
                    (None, "description") => {
                        n.rss_channel_item_description_elems
                            .push(RssChannelItemDescription::parse_children(attributes, iter));
                    }
                    (None, "enclosure") => {
                        n.enclosure_elems
                            .push(Enclosure::parse_children(attributes, iter));
                    }
                    (None, "guid") => {
                        n.guid_elems.push(Guid::parse_children(attributes, iter));
                    }
                    (None, "pubDate") => {
                        n.pub_date_elems
                            .push(PubDate::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "duration") => {
                        n.duration_elems
                            .push(Duration::parse_children(attributes, iter));
                    }
                    (Some("http://www.itunes.com/dtds/podcast-1.0.dtd"), "explicit") => {
                        n.rss_channel_item_itunes_explicit_elems.push(
                            RssChannelItemItunesExplicit::parse_children(attributes, iter),
                        );
                    }
                    _ => {
                        todo!();
                    }
                },
                Ok(xml::reader::XmlEvent::EndElement { .. }) => {
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {}
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EpisodeType {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl EpisodeType {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemItunesTitle {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItemItunesTitle {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemDescription {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItemDescription {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
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
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {}
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Guid {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Guid {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PubDate {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl PubDate {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Duration {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Duration {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemItunesExplicit {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItemItunesExplicit {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Episode {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Episode {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Season {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl Season {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemTitle {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItemTitle {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemItunesImage {
    pub r#href: Option<String>,
    pub misc_attrs: HashMap<(Option<String>, String), String>,
}
impl RssChannelItemItunesImage {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {}
                _ => {}
            }
        }
        n
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemLink {
    pub misc_attrs: HashMap<(Option<String>, String), String>,
    pub value: Option<String>,
}
impl RssChannelItemLink {
    pub fn parse_children<T: std::io::Read>(
        attrs: Vec<xml::attribute::OwnedAttribute>,
        iter: &mut xml::reader::Events<T>,
    ) -> Self {
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
                    return n;
                }
                Ok(xml::reader::XmlEvent::Characters(val)) => {
                    n.value = Some(val);
                }
                _ => {}
            }
        }
        n
    }
}
