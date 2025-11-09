use crate::RssDocument;
use std::io::Cursor;

#[test]
fn parse_and_check_important_values() {
    let podcast1_xml = include_bytes!("podcast1.xml");
    let podcast1_xml_cursor = Cursor::new(podcast1_xml);

    let podcast1 = RssDocument::parse_document(podcast1_xml_cursor);

    assert_eq!(1, podcast1.channel_elems.len());

    let channel = &podcast1.channel_elems[0];

    assert_eq!(9, channel.item_elems.len());

    let item_0 = &channel.item_elems[0];
    let item_0_enc = &item_0.enclosure_elems[0];

    assert_eq!(Some("498537"), item_0_enc.length.as_deref());
    assert_eq!(Some("audio/mpeg"), item_0_enc.r#type.as_deref());
}
