use thiserror::Error;
use xml::name::OwnedName;

#[derive(Debug, Error)]
pub enum XmlDocumentReference {
    #[error("Unknown reference")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum XmlParseError {
    #[error(transparent)]
    XmlRsRead(#[from] xml::reader::Error),
    #[error("Expected EndElement")]
    ExpectedEndElement(XmlDocumentReference),
    #[error("UnexpectedCharacters")]
    UnexpectedCharacters(XmlDocumentReference),
    #[error("Encountered unexpected Element: {0}")]
    UnexpectedElement(OwnedName),
    #[error("Encountered unexpected XMLEvent: {0:?}")]
    UnexpectedXmlEvent(xml::reader::XmlEvent),
    #[error("Encountered unexpected EOF. Expecting: {0}")]
    UnexpectedEof(&'static str),
    #[error("Expected EOF, encountered: {0:?}")]
    ExpectedEof(xml::reader::XmlEvent),
}

#[derive(Error, Debug)]
pub enum XmlWriteError {
    #[error(transparent)]
    XmlRsWrite(#[from] xml::writer::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("UTF8 Error")]
    Utf8(#[from] std::str::Utf8Error),
}
