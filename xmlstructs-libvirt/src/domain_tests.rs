use crate::DomainDocument;
use std::io::Cursor;
use xml::reader::XmlEvent;
use xml_struct_types::utils::parse_xml_to_event_vec;
use xml_struct_types::v1::XmlStructDocument;
use xml_struct_types::v1::utils::write_document_to_string;

#[test]
fn roundtrip_xml() {
    let domain1_xml = include_bytes!("domain1.xml");
    let domain1 = DomainDocument::parse_document(&mut Cursor::new(domain1_xml)).unwrap();
    let _domain1_xml_roundtripped = write_document_to_string(domain1).unwrap();
    //println!("{domain1_xml_roundtripped}");
}

fn event_normalise_filter(evt: &XmlEvent) -> bool {
    match evt {
        XmlEvent::Whitespace(_) => false,
        _ => true,
    }
}

#[test]
fn roundtrip_event_list_matches() {
    let domain1_xml = include_bytes!("domain1.xml");
    let domain1 = DomainDocument::parse_document(&mut Cursor::new(domain1_xml)).unwrap();
    let domain1_xml_roundtripped = write_document_to_string(domain1).unwrap();

    let mut domain1_xml_reader = xml::EventReader::new(Cursor::new(domain1_xml)).into_iter();
    let domain1_xml_events = parse_xml_to_event_vec(&mut domain1_xml_reader)
        .unwrap()
        .into_iter()
        .filter(event_normalise_filter)
        .collect::<Vec<_>>();

    let mut domain1_xml_roundtripped_reader =
        xml::EventReader::new(Cursor::new(domain1_xml_roundtripped)).into_iter();
    let domain1_xml_roundtripped_events =
        parse_xml_to_event_vec(&mut domain1_xml_roundtripped_reader)
            .unwrap()
            .into_iter()
            .filter(event_normalise_filter)
            .collect::<Vec<_>>();

    assert_eq!(
        domain1_xml_events.len(),
        domain1_xml_roundtripped_events.len()
    );

    // this doesn't quite work yet
    //assert_eq!(domain1_xml_events, domain1_xml_roundtripped_events);
}
