use crate::v1::error::{XmlParseError, XmlWriteError};

pub trait XmlStructDocument {
    fn parse_document<R: std::io::Read>(reader: &mut R) -> Result<Self, XmlParseError>
    where
        Self: Sized;

    fn write_document<W: std::io::Write>(&self, w: &mut W) -> Result<(), XmlWriteError>;
}
