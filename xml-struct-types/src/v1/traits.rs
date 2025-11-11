use crate::v1::error::{XmlParseError, XmlWriteError};

pub trait XmlStructDocument {
    fn parse_document<R: std::io::Read>(reader: R) -> Result<Self, XmlParseError>
    where
        Self: Sized;
    fn write_document<W: std::io::Write>(
        &self,
        w: &mut xml::writer::EventWriter<W>,
    ) -> Result<(), XmlWriteError>;
}
