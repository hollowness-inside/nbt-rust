use crate::error::Error;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum TagType {
    End = 0x00,
    Byte = 0x01,
    Short = 0x02,
    Int = 0x03,
    Long = 0x04,
    Float = 0x05,
    Double = 0x06,
    ByteArray = 0x07,
    String = 0x08,
    List = 0x09,
    Compound = 0x0a,
    IntArray = 0x0b,
    LongArray = 0x0c,
}

impl TryFrom<u8> for TagType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(TagType::End),
            0x01 => Ok(TagType::Byte),
            0x02 => Ok(TagType::Short),
            0x03 => Ok(TagType::Int),
            0x04 => Ok(TagType::Long),
            0x05 => Ok(TagType::Float),
            0x06 => Ok(TagType::Double),
            0x07 => Ok(TagType::ByteArray),
            0x08 => Ok(TagType::String),
            0x09 => Ok(TagType::List),
            0x0a => Ok(TagType::Compound),
            0x0b => Ok(TagType::IntArray),
            0x0c => Ok(TagType::LongArray),
            _ => Err(Error::UnknownTagType(value)),
        }
    }
}
