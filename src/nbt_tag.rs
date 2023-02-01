use std::{fmt::{self, Formatter}, collections::HashMap};

use crate::error::Error;

/// In the binary format, each tag is prefixed with a single byte
/// which identifies its type. The tag prefixes are listed below.
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

/// The NbtTag enum represents all the possible NBT tags.
#[derive(Clone)]
pub enum NbtTag {
    End,
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<NbtTag>),
    Compound(HashMap<String, NbtTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NbtTag {
    /// Returns the tag prefix of the tag.
    pub const fn tag_prefix(&self) -> TagType {
        match self {
            NbtTag::End => TagType::End,
            NbtTag::Byte(_) => TagType::Byte,
            NbtTag::Short(_) => TagType::Short,
            NbtTag::Int(_) => TagType::Int,
            NbtTag::Long(_) => TagType::Long,
            NbtTag::Float(_) => TagType::Float,
            NbtTag::Double(_) => TagType::Double,
            NbtTag::ByteArray(_) => TagType::ByteArray,
            NbtTag::String(_) => TagType::String,
            NbtTag::List(_) => TagType::List,
            NbtTag::Compound(_) => TagType::Compound,
            NbtTag::IntArray(_) => TagType::IntArray,
            NbtTag::LongArray(_) => TagType::LongArray,
        }
    }
}

impl fmt::Display for NbtTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            NbtTag::End => write!(f, "End"),
            NbtTag::Byte(v) => write!(f, "{v}b"),
            NbtTag::Short(v) => write!(f, "{v}s"),
            NbtTag::Int(v) => write!(f, "{v}"),
            NbtTag::Long(v) => write!(f, "{v}l"),
            NbtTag::Float(v) => write!(f, "{v}f"),
            NbtTag::Double(v) => write!(f, "{v}d"),
            NbtTag::String(v) => write!(f, "\"{v}\""),
            NbtTag::ByteArray(v) => {
                write!(f, "[B; ")?;
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{v}")?;
                }
                write!(f, "]")
            }
            NbtTag::List(v) => {
                write!(f, "[")?;
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{v}")?;
                }
                write!(f, "]")
            }
            NbtTag::IntArray(v) => {
                write!(f, "[I; ")?;
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{v}")?;
                }
                write!(f, "]")
            }
            NbtTag::LongArray(v) => {
                write!(f, "[L; ")?;
                for (i, v) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{v}")?;
                }
                write!(f, "]")
            }
            NbtTag::Compound(v) => {
                write!(f, "{{")?;
                for (i, (k, v)) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{k}\": {v}")?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl From<u8> for NbtTag {
    fn from(v: u8) -> Self {
        NbtTag::Byte(v)
    }
}

impl From<i8> for NbtTag {
    fn from(v: i8) -> Self {
        NbtTag::Byte(v as u8)
    }
}

impl From<i16> for NbtTag {
    fn from(v: i16) -> Self {
        NbtTag::Short(v)
    }
}

impl From<u16> for NbtTag {
    fn from(v: u16) -> Self {
        NbtTag::Short(v as i16)
    }
}

impl From<i32> for NbtTag {
    fn from(v: i32) -> Self {
        NbtTag::Int(v)
    }
}

impl From<u32> for NbtTag {
    fn from(v: u32) -> Self {
        NbtTag::Int(v as i32)
    }
}

impl From<i64> for NbtTag {
    fn from(v: i64) -> Self {
        NbtTag::Long(v)
    }
}

impl From<u64> for NbtTag {
    fn from(v: u64) -> Self {
        NbtTag::Long(v as i64)
    }
}

impl From<f32> for NbtTag {
    fn from(v: f32) -> Self {
        NbtTag::Float(v)
    }
}

impl From<f64> for NbtTag {
    fn from(v: f64) -> Self {
        NbtTag::Double(v)
    }
}

impl From<Vec<u8>> for NbtTag {
    fn from(v: Vec<u8>) -> Self {
        NbtTag::ByteArray(v)
    }
}

impl From<String> for NbtTag {
    fn from(v: String) -> Self {
        NbtTag::String(v)
    }
}

impl From<Vec<NbtTag>> for NbtTag {
    fn from(v: Vec<NbtTag>) -> Self {
        NbtTag::List(v)
    }
}

impl From<Vec<(String, NbtTag)>> for NbtTag {
    fn from(v: Vec<(String, NbtTag)>) -> Self {
        let h: HashMap<String, NbtTag> = v.into_iter().collect();
        NbtTag::Compound(h)
    }
}

impl From<HashMap<String, NbtTag>> for NbtTag {
    fn from(v: HashMap<String, NbtTag>) -> Self {
        NbtTag::Compound(v)
    }
}

impl From<Vec<i32>> for NbtTag {
    fn from(v: Vec<i32>) -> Self {
        NbtTag::IntArray(v)
    }
}

impl From<Vec<i64>> for NbtTag {
    fn from(v: Vec<i64>) -> Self {
        NbtTag::LongArray(v)
    }
}

impl From<()> for NbtTag {
    fn from(_: ()) -> Self {
        NbtTag::End
    }
}
