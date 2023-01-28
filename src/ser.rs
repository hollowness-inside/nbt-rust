use std::io;

use crate::{
    error::{Error, Result},
    nbt_tag::prefixes,
    NbtTag,
};

pub struct Serializer<W> {
    writer: W,
}

impl<W: io::Write> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }

    pub fn serialize<T: Into<NbtTag>>(&mut self, v: T) -> Result<()> {
        self.serialize_tag(&v.into())?;
        Ok(())
    }

    pub fn serialize_tag(&mut self, v: &NbtTag) -> Result<()> {
        match v {
            NbtTag::End => self.write_end(),
            NbtTag::Byte(v) => self.serialize_u8(*v),
            NbtTag::Short(v) => self.serialize_i16(*v),
            NbtTag::Int(v) => self.serialize_i32(*v),
            NbtTag::Long(v) => self.serialize_i64(*v),
            NbtTag::Float(v) => self.serialize_f32(*v),
            NbtTag::Double(v) => self.serialize_f64(*v),
            NbtTag::ByteArray(v) => self.serialize_byte_array(v),
            NbtTag::String(v) => self.serialize_str(v),
            NbtTag::List(v) => self.serialize_list(v),
            NbtTag::Compound(v) => self.serialize_compound(v),
            NbtTag::IntArray(v) => self.serialize_int_array(v),
            NbtTag::LongArray(v) => self.serialize_long_array(v),
        }
    }

    pub fn write_end(&mut self) -> Result<()> {
        self.writer.write_all(&[prefixes::END])?;
        Ok(())
    }

    pub fn serialize_bool(&mut self, v: bool) -> Result<()> {
        self.serialize_u8(if v { 1 } else { 0 })
    }

    pub fn serialize_u8(&mut self, v: u8) -> Result<()> {
        self.writer.write_all(&[prefixes::BYTE])?;
        self.writer.write_all(&[v])?;
        Ok(())
    }

    pub fn serialize_i8(&mut self, v: i8) -> Result<()> {
        self.serialize_u8(v as u8)
    }

    pub fn serialize_u16(&mut self, v: u16) -> Result<()> {
        self.writer.write_all(&[prefixes::SHORT])?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    pub fn serialize_i16(&mut self, v: i16) -> Result<()> {
        self.serialize_u16(v as u16)
    }

    pub fn serialize_u32(&mut self, v: u32) -> Result<()> {
        self.writer.write_all(&[prefixes::INT])?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    pub fn serialize_i32(&mut self, v: i32) -> Result<()> {
        self.serialize_u32(v as u32)
    }

    pub fn serialize_u64(&mut self, v: u64) -> Result<()> {
        self.writer.write_all(&[prefixes::LONG])?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    pub fn serialize_i64(&mut self, v: i64) -> Result<()> {
        self.serialize_u64(v as u64)
    }

    pub fn serialize_f32(&mut self, v: f32) -> Result<()> {
        self.writer.write_all(&[prefixes::FLOAT])?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    pub fn serialize_f64(&mut self, v: f64) -> Result<()> {
        self.writer.write_all(&[prefixes::DOUBLE])?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    pub fn serialize_char(&mut self, v: char) -> Result<()> {
        self.serialize_u8(v as u8)
    }

    pub fn serialize_str(&mut self, v: &str) -> Result<()> {
        self.writer.write_all(&[prefixes::STRING])?;
        self.writer.write_all(&(v.len() as i16).to_be_bytes())?;
        self.writer.write_all(v.as_bytes())?;
        Ok(())
    }

    pub fn serialize_compound(&mut self, v: &[(String, NbtTag)]) -> Result<()> {
        self.writer.write_all(&[prefixes::COMPOUND])?;
        for (name, tag) in v {
            self.serialize_str(name)?;
            self.serialize_tag(tag)?;
        }
        self.writer.write_all(&[0])?;
        Ok(())
    }

    pub fn serialize_byte_array(&mut self, v: &[u8]) -> Result<()> {
        self.writer.write_all(&[prefixes::BYTE_ARRAY])?;
        self.writer.write_all(&(v.len() as i32).to_be_bytes())?;
        self.writer.write_all(v)?;
        Ok(())
    }

    pub fn serialize_list(&mut self, value: &[NbtTag]) -> Result<()> {
        let tag_type = value.first().map(|t| t.tag_type()).unwrap_or(0);
        if !value.iter().all(|x| x.tag_type() == tag_type) {
            return Err(Error::Generic(
                "All elements in a list must have the same type".to_string(),
            ));
        }

        self.writer.write_all(&[prefixes::LIST])?;
        self.writer.write_all(&[tag_type])?;
        self.writer.write_all(&(value.len() as i32).to_be_bytes())?;

        match value.first() {
            Some(NbtTag::Byte(v)) => {
                for _i in value {
                    self.serialize_u8(*v)?;
                }
            }
            Some(NbtTag::Short(v)) => {
                for _i in value {
                    self.serialize_i16(*v)?;
                }
            }
            Some(NbtTag::Int(v)) => {
                for _i in value {
                    self.serialize_i32(*v)?;
                }
            }
            Some(NbtTag::Long(v)) => {
                for _i in value {
                    self.serialize_i64(*v)?;
                }
            }
            Some(NbtTag::Float(v)) => {
                for _i in value {
                    self.serialize_f32(*v)?;
                }
            }
            Some(NbtTag::Double(v)) => {
                for _i in value {
                    self.serialize_f64(*v)?;
                }
            }
            Some(NbtTag::ByteArray(v)) => {
                for _i in value {
                    self.serialize_byte_array(v)?;
                }
            }
            Some(NbtTag::String(v)) => {
                for _i in value {
                    self.serialize_str(v)?;
                }
            }
            Some(NbtTag::List(v)) => {
                for _i in value {
                    self.serialize_list(v)?;
                }
            }
            Some(NbtTag::Compound(v)) => {
                for _i in value {
                    self.serialize_compound(v)?;
                }
            }
            Some(NbtTag::IntArray(v)) => {
                for _i in value {
                    self.serialize_int_array(v)?;
                }
            }
            Some(NbtTag::LongArray(v)) => {
                for _i in value {
                    self.serialize_long_array(v)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn serialize_int_array(&mut self, v: &[i32]) -> Result<()> {
        self.writer.write_all(&[prefixes::INT_ARRAY])?;
        self.writer.write_all(&(v.len() as i32).to_be_bytes())?;
        for i in v {
            self.writer.write_all(&i.to_be_bytes())?;
        }
        Ok(())
    }

    pub fn serialize_long_array(&mut self, v: &[i64]) -> Result<()> {
        self.writer.write_all(&[prefixes::LONG_ARRAY])?;
        self.writer.write_all(&(v.len() as i32).to_be_bytes())?;
        for i in v {
            self.writer.write_all(&i.to_be_bytes())?;
        }
        Ok(())
    }
}
