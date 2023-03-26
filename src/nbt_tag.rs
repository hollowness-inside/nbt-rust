use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use crate::error::Error;

macro_rules! impl_from {
    ($from:ty, $var:ident) => {
        impl From<$from> for NbtTag {
            fn from(v: $from) -> Self {
                NbtTag::$var(v)
            }
        }
    };
    ($from:ty as $v:ty, $var:ident) => {
        impl From<$from> for NbtTag {
            fn from(v: $from) -> Self {
                NbtTag::$var(v as $v)
            }
        }
    };
}

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
    pub const fn tag_type(&self) -> TagType {
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

impl_from!(u8, Byte);
impl_from!(i8 as u8, Byte);
impl_from!(i16, Short);
impl_from!(u16 as i16, Short);
impl_from!(i32, Int);
impl_from!(u32 as i32, Int);
impl_from!(i64, Long);
impl_from!(u64 as i64, Long);
impl_from!(f32, Float);
impl_from!(f64, Double);
impl_from!(Vec<u8>, ByteArray);
impl_from!(String, String);
impl_from!(Vec<NbtTag>, List);
impl_from!(HashMap<String, NbtTag>, Compound);
impl_from!(Vec<i32>, IntArray);
impl_from!(Vec<i64>, LongArray);

impl From<Vec<(String, NbtTag)>> for NbtTag {
    fn from(v: Vec<(String, NbtTag)>) -> Self {
        let h: HashMap<String, NbtTag> = v.into_iter().collect();
        NbtTag::Compound(h)
    }
}
