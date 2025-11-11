use crate::v1::error::XmlParseError;

pub trait XmlStructDocument {
    fn parse_document<R: std::io::Read>(reader: R) -> Result<Self, XmlParseError>
    where
        Self: Sized;
}
