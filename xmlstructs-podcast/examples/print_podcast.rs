use std::io::stdout;
use xml_struct_types::v1::XmlStructDocument;
use xmlstructs_podcast::{Channel, RssChannelTitle, RssDocument};

fn main() {
    let rss_doc = RssDocument {
        channel_elems: vec![Channel {
            rss_channel_title_elems: vec![RssChannelTitle {
                value: Some("My awesome podcast".into()),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    rss_doc.write_document(&mut stdout()).unwrap();
}
