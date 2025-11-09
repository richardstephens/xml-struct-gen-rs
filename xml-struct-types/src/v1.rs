use thiserror::Error;

#[derive(Error, Debug)]
pub enum XmlParseError {
    #[error(transparent)]
    XmlRsRead(#[from] xml::reader::Error),
    #[error("Expected EndElement")]
    ExpectedEndElement,
}
