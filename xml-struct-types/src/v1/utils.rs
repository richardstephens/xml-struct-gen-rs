use crate::v1::XmlStructDocument;
use crate::v1::error::XmlWriteError;
use std::io::{Cursor, Read, Seek, SeekFrom};

pub fn write_document_to_string(doc: impl XmlStructDocument) -> Result<String, XmlWriteError> {
    let mut c = Cursor::new(Vec::new());
    doc.write_document(&mut c)?;

    c.seek(SeekFrom::Start(0))?;
    let mut out = Vec::new();
    c.read_to_end(&mut out)?;
    Ok(String::from_utf8(out).map_err(|e| e.utf8_error())?)
}
