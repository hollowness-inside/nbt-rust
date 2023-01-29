use std::io::{Cursor, Write};

use crate::{
    error::{Error, Result},
    nbt_tag::prefixes,
    NbtTag,
};

pub struct Serializer;

impl Serializer {
    pub fn serialize<T: Into<NbtTag> + Clone>(v: &T) -> Result<Vec<u8>> {
        Self::serialize_tag(&v.clone().into())
    }

    pub fn serialize_tag(v: &NbtTag) -> Result<Vec<u8>> {
        match v {
            NbtTag::End => Self::serialize_end(),
            NbtTag::Byte(v) => Self::serialize_u8(*v),
            NbtTag::Short(v) => Self::serialize_i16(*v),
            NbtTag::Int(v) => Self::serialize_i32(*v),
            NbtTag::Long(v) => Self::serialize_i64(*v),
            NbtTag::Float(v) => Self::serialize_f32(*v),
            NbtTag::Double(v) => Self::serialize_f64(*v),
            NbtTag::ByteArray(v) => Self::serialize_byte_array(v),
            NbtTag::String(v) => Self::serialize_str(v),
            NbtTag::List(v) => Self::serialize_list(v),
            NbtTag::Compound(v) => Self::serialize_compound(v),
            NbtTag::IntArray(v) => Self::serialize_int_array(v),
            NbtTag::LongArray(v) => Self::serialize_long_array(v),
        }
    }

    pub fn serialize_end() -> Result<Vec<u8>> {
        Ok(vec![prefixes::END])
    }

    pub fn serialize_bool(v: bool) -> Result<Vec<u8>> {
        Self::serialize_u8(if v { 1 } else { 0 })
    }

    pub fn serialize_u8(v: u8) -> Result<Vec<u8>> {
        Ok(vec![prefixes::BYTE, v])
    }

    pub fn serialize_i8(v: i8) -> Result<Vec<u8>> {
        Self::serialize_u8(v as u8)
    }

    pub fn serialize_u16(v: u16) -> Result<Vec<u8>> {
        Ok(vec![vec![prefixes::SHORT], v.to_be_bytes().to_vec()].concat())
    }

    pub fn serialize_i16(v: i16) -> Result<Vec<u8>> {
        Self::serialize_u16(v as u16)
    }

    pub fn serialize_u32(v: u32) -> Result<Vec<u8>> {
        Ok(vec![vec![prefixes::INT], v.to_be_bytes().to_vec()].concat())
    }

    pub fn serialize_i32(v: i32) -> Result<Vec<u8>> {
        Self::serialize_u32(v as u32)
    }

    pub fn serialize_u64(v: u64) -> Result<Vec<u8>> {
        Ok(vec![vec![prefixes::LONG], v.to_be_bytes().to_vec()].concat())
    }

    pub fn serialize_i64(v: i64) -> Result<Vec<u8>> {
        Self::serialize_u64(v as u64)
    }

    pub fn serialize_f32(v: f32) -> Result<Vec<u8>> {
        Ok(vec![vec![prefixes::FLOAT], v.to_be_bytes().to_vec()].concat())
    }

    pub fn serialize_f64(v: f64) -> Result<Vec<u8>> {
        Ok(vec![vec![prefixes::DOUBLE], v.to_be_bytes().to_vec()].concat())
    }

    pub fn serialize_char(v: char) -> Result<Vec<u8>> {
        Self::serialize_u8(v as u8)
    }

    pub fn serialize_str(v: &str) -> Result<Vec<u8>> {
        Ok(vec![
            vec![prefixes::STRING],
            (v.len() as i16).to_be_bytes().to_vec(),
            v.as_bytes().to_vec(),
        ]
        .concat())
    }

    pub fn serialize_compound<S: ToString>(v: &[(S, NbtTag)]) -> Result<Vec<u8>> {
        let mut writer = Cursor::new(Vec::new());
        writer.write_all(&[prefixes::COMPOUND])?;
        for (name, tag) in v {
            writer.write_all(&Self::serialize_str(&name.to_string())?)?;
            writer.write_all(&Self::serialize_tag(tag)?)?;
        }
        writer.write_all(&[prefixes::END])?;

        Ok(writer.into_inner())
    }

    pub fn serialize_byte_array(v: &[u8]) -> Result<Vec<u8>> {
        Ok(vec![
            vec![prefixes::BYTE_ARRAY],
            (v.len() as i32).to_be_bytes().to_vec(),
            v.to_vec(),
        ]
        .concat())
    }

    pub fn serialize_list(value: &[NbtTag]) -> Result<Vec<u8>> {
        let tag_type = value.first().map(|t| t.tag_type()).unwrap_or(0);
        if !value.iter().all(|x| x.tag_type() == tag_type) {
            return Err(Error::Generic(
                "All elements in a list must have the same type".to_string(),
            ));
        }

        let mut writer = Cursor::new(Vec::new());

        writer.write_all(&[prefixes::LIST])?;
        writer.write_all(&[tag_type])?;
        writer.write_all(&(value.len() as i32).to_be_bytes())?;

        match value.first() {
            Some(NbtTag::Byte(v)) => {
                for _i in value {
                    Self::serialize_u8(*v)?;
                }
            }
            Some(NbtTag::Short(v)) => {
                for _i in value {
                    Self::serialize_i16(*v)?;
                }
            }
            Some(NbtTag::Int(v)) => {
                for _i in value {
                    Self::serialize_i32(*v)?;
                }
            }
            Some(NbtTag::Long(v)) => {
                for _i in value {
                    Self::serialize_i64(*v)?;
                }
            }
            Some(NbtTag::Float(v)) => {
                for _i in value {
                    Self::serialize_f32(*v)?;
                }
            }
            Some(NbtTag::Double(v)) => {
                for _i in value {
                    Self::serialize_f64(*v)?;
                }
            }
            Some(NbtTag::ByteArray(v)) => {
                for _i in value {
                    Self::serialize_byte_array(v)?;
                }
            }
            Some(NbtTag::String(v)) => {
                for _i in value {
                    Self::serialize_str(v)?;
                }
            }
            Some(NbtTag::List(v)) => {
                for _i in value {
                    Self::serialize_list(v)?;
                }
            }
            Some(NbtTag::Compound(v)) => {
                for _i in value {
                    Self::serialize_compound(v)?;
                }
            }
            Some(NbtTag::IntArray(v)) => {
                for _i in value {
                    Self::serialize_int_array(v)?;
                }
            }
            Some(NbtTag::LongArray(v)) => {
                for _i in value {
                    Self::serialize_long_array(v)?;
                }
            }
            _ => {}
        }

        Ok(writer.into_inner())
    }

    pub fn serialize_int_array(v: &[i32]) -> Result<Vec<u8>> {
        let mut writer = Cursor::new(Vec::new());
        writer.write_all(&[prefixes::INT_ARRAY])?;
        writer.write_all(&(v.len() as i32).to_be_bytes())?;
        for i in v {
            writer.write_all(&i.to_be_bytes())?;
        }
        Ok(writer.into_inner())
    }

    pub fn serialize_long_array(v: &[i64]) -> Result<Vec<u8>> {
        let mut writer = Cursor::new(Vec::new());
        writer.write_all(&[prefixes::LONG_ARRAY])?;
        writer.write_all(&(v.len() as i32).to_be_bytes())?;
        for i in v {
            writer.write_all(&i.to_be_bytes())?;
        }
        Ok(writer.into_inner())
    }

    pub fn start_compound() -> CompoundSerializer {
        CompoundSerializer {
            is_first: true,
            output: Vec::new(),
        }
    }
}

pub struct CompoundSerializer {
    is_first: bool,
    output: Vec<u8>,
}

impl CompoundSerializer {
    pub fn write_field<T: Into<NbtTag> + Clone>(&mut self, key: &str, value: &T) -> Result<()> {
        if self.is_first {
            self.output.push(prefixes::COMPOUND);
            self.is_first = false;
        }

        self.output.extend(Serializer::serialize_str(&key)?);
        self.output.extend(Serializer::serialize(value)?);
        Ok(())
    }

    pub fn end(mut self) -> Vec<u8> {
        self.output.push(prefixes::END);
        self.output
    }
}
