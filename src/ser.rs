use std::io;

use crate::{nbt_tag::prefixes, error::Result};

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

    fn serialize_tag(&mut self, v: &NbtTag) -> Result<()> {
        match v {
            NbtTag::End => self.write_end(),
            NbtTag::Byte(v) => self.serialize_u8(*v),
            NbtTag::Short(v) => self.serialize_i16(*v),
            NbtTag::Int(v) => self.serialize_i32(*v),
            NbtTag::Long(v) => self.serialize_i64(*v),
            NbtTag::Float(v) => self.serialize_f32(*v),
            NbtTag::Double(v) => self.serialize_f64(*v),
            NbtTag::ByteArray(v) => self.serialize_byte_array(&v),
            NbtTag::String(v) => self.serialize_str(&v),
            NbtTag::List(v) => self.serialize_list(&v),
            NbtTag::Compound(v) => self.serialize_compound(&v),
            NbtTag::IntArray(v) => self.serialize_int_array(&v),
            NbtTag::LongArray(v) => self.serialize_long_array(&v),
        }
    }

    fn write_end(&mut self) -> Result<()> {
        self.writer.write_all(&[prefixes::END])?;
        Ok(())
    }

    fn serialize_bool(&mut self, v: bool) -> Result<()> {
        self.serialize_u8(if v { 1 } else { 0 })
    }

    fn serialize_u8(&mut self, v: u8) -> Result<()> {
        self.writer.write_all(&[prefixes::BYTE])?;
        self.writer.write_all(&[v])?;
        Ok(())
    }

    fn serialize_i8(&mut self, v: i8) -> Result<()> {
        self.serialize_u8(v as u8)
    }

    fn serialize_u16(&mut self, v: u16) -> Result<()> {
        self.writer.write_all(&[prefixes::SHORT])?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i16(&mut self, v: i16) -> Result<()> {
        self.serialize_u16(v as u16)
    }

    fn serialize_u32(&mut self, v: u32) -> Result<()> {
        self.writer.write_all(&[prefixes::INT])?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i32(&mut self, v: i32) -> Result<()> {
        self.serialize_u32(v as u32)
    }

    fn serialize_u64(&mut self, v: u64) -> Result<()> {
        self.writer.write_all(&[prefixes::LONG])?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i64(&mut self, v: i64) -> Result<()> {
        self.serialize_u64(v as u64)
    }

    fn serialize_f32(&mut self, v: f32) -> Result<()> {
        self.writer.write_all(&[prefixes::FLOAT])?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_f64(&mut self, v: f64) -> Result<()> {
        self.writer.write_all(&[prefixes::DOUBLE])?;
        self.writer.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_char(&mut self, v: char) -> Result<()> {
        self.serialize_u8(v as u8)
    }

    fn serialize_str(&mut self, v: &str) -> Result<()> {
        self.writer.write_all(&[prefixes::STRING])?;
        self.writer.write_all(&(v.len() as i16).to_be_bytes())?;
        self.writer.write_all(v.as_bytes())?;
        Ok(())
    }

    fn serialize_compound(&mut self, v: &[(String, NbtTag)]) -> Result<()> {
        self.writer.write_all(&[prefixes::COMPOUND])?;
        for (name, tag) in v {
            self.serialize_str(name)?;
            self.serialize_tag(tag)?;
        }
        self.writer.write_all(&[0])?;
        Ok(())
    }

        self.writer.write_all(&[prefixes::BYTE_ARRAY])?;
        self.writer.write_all(&(v.len() as i32).to_be_bytes())?;
        self.writer.write_all(v)?;
        Ok(())
    }
}
