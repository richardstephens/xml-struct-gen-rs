/// helper fn intended for writing tests
/// Read an XmlReader until EOF, return all events in a Vec.
pub fn parse_xml_to_event_vec<R: std::io::Read>(
    iter: &mut xml::reader::Events<R>,
) -> Result<Vec<xml::reader::XmlEvent>, xml::reader::Error> {
    let mut events = Vec::new();
    while let Some(event) = iter.next() {
        match event {
            Ok(e) => events.push(e),
            Err(e) => return Err(e),
        };
    }
    Ok(events)
}
