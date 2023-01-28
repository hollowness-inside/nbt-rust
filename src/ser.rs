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

    fn serialize_bytes(&mut self, v: &[u8]) -> Result<()> {
        self.writer.write_all(&[prefixes::BYTE_ARRAY])?;
        self.writer.write_all(&(v.len() as i32).to_be_bytes())?;
        self.writer.write_all(v)?;
        Ok(())
    }
}
