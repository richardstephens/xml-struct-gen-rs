use crate::RssDocument;
use std::io::Cursor;

#[test]
fn parse_and_check_important_values() {
    let podcast1_xml = include_bytes!("podcast1.xml");
    let podcast1_xml_cursor = Cursor::new(podcast1_xml);

    let podcast1 = RssDocument::parse_document(podcast1_xml_cursor);

    assert_eq!(1, podcast1.channel_elems.len());

    let channel = &podcast1.channel_elems[0];

    // check text fields
    assert_eq!(
        Some("Hiking Treks"),
        channel.rss_channel_title_elems[0].value.as_deref()
    );
    assert_eq!(Some("en-us"), channel.language_elems[0].value.as_deref());
    // check some of the "itunes" namespaced fields
    assert_eq!(Some("serial"), channel.type_elems[0].value.as_deref());

    assert_eq!(9, channel.item_elems.len());

    let item_0 = &channel.item_elems[0];
    let item_0_enc = &item_0.enclosure_elems[0];

    // check attribute values
    assert_eq!(Some("498537"), item_0_enc.length.as_deref());
    assert_eq!(Some("audio/mpeg"), item_0_enc.r#type.as_deref());

    // check that the last couple of items looks right,
    // to ensure we didn't get de-synced somehow
    let item_8 = &channel.item_elems[8];
    let item_8_guid = item_8.guid_elems.get(0).unwrap().value.as_deref();
    assert_eq!(Some("EABDA7EE-1AC6-4B60-9E11-6B3F30B72F87"), item_8_guid);
}
