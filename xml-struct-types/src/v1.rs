use thiserror::Error;

#[derive(Debug)]
pub enum XmlDocumentPosition {
    Unknown,
}

#[derive(Error, Debug)]
pub enum XmlParseError {
    #[error(transparent)]
    XmlRsRead(#[from] xml::reader::Error),
    #[error("Expected EndElement")]
    ExpectedEndElement(XmlDocumentPosition),
    #[error("UnexpectedCharacters")]
    UnexpectedCharacters(XmlDocumentPosition),
}

#[derive(Error, Debug)]
pub enum XmlWriteError {
    #[error(transparent)]
    XmlRsWrite(#[from] xml::writer::Error),
}
