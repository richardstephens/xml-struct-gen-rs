use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssDocument {
    pub r#version: Option<String>,
    pub channel_elems: Vec<Channel>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Channel {
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
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelTitle {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelLink {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Language {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Copyright {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelDescription {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Type {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesImage {
    pub r#href: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesCategory {
    pub r#text: Option<String>,
    pub rss_channel_itunes_category_itunes_category_elems:
        Vec<RssChannelItunesCategoryItunesCategory>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesCategoryItunesCategory {
    pub r#text: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItunesExplicit {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
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
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EpisodeType {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemItunesTitle {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemDescription {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Enclosure {
    pub r#length: Option<String>,
    pub r#type: Option<String>,
    pub r#url: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Guid {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PubDate {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Duration {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemItunesExplicit {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Episode {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Season {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemTitle {
    pub value: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemItunesImage {
    pub r#href: Option<String>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RssChannelItemLink {
    pub value: Option<String>,
}
