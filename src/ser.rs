use std::io;

use serde::ser;

use crate::{error::{Result, Error}, NbtTag};

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

    fn serialize_tag(&mut self, tag: &NbtTag) -> Result<()> {
        match tag {
            NbtTag::End => self.serialize_end(),
            NbtTag::Byte(value) => self.serialize_byte(value),
            NbtTag::Short(value) => self.serialize_short(value),
            NbtTag::Int(value) => self.serialize_int(value),
            NbtTag::Long(value) => self.serialize_long(value),
            NbtTag::Float(value) => self.serialize_float(value),
            NbtTag::Double(value) => self.serialize_double(value),
            NbtTag::ByteArray(value) => self.serialize_byte_array(value),
            NbtTag::String(value) => self.serialize_string(value),
            NbtTag::List(value) => self.serialize_list(value),
            NbtTag::Compound(value) => self.serialize_compound(value),
            NbtTag::IntArray(value) => self.serialize_int_array(value),
            NbtTag::LongArray(value) => self.serialize_long_array(value),
        }
    }

    #[inline]
    fn serialize_end(&mut self) -> Result<()> {
        self.writer.write_all(&[0x00])?;
        Ok(())
    }

    #[inline]
    fn serialize_byte(&mut self, value: &u8) -> Result<()> {
        self.writer.write_all(&[0x01])?;
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    #[inline]
    fn serialize_short(&mut self, value: &i16) -> Result<()> {
        self.writer.write_all(&[0x02])?;
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    #[inline]
    fn serialize_int(&mut self, value: &i32) -> Result<()> {
        self.writer.write_all(&[0x03])?;
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    #[inline]
    fn serialize_long(&mut self, value: &i64) -> Result<()> {
        self.writer.write_all(&[0x04])?;
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    #[inline]
    fn serialize_float(&mut self, value: &f32) -> Result<()> {
        self.writer.write_all(&[0x05])?;
        self.writer.write_all(&value.to_bits().to_be_bytes())?;
        Ok(())
    }

    #[inline]
    fn serialize_double(&mut self, value: &f64) -> Result<()> {
        self.writer.write_all(&[0x06])?;
        self.writer.write_all(&value.to_bits().to_be_bytes())?;
        Ok(())
    }

    fn serialize_byte_array(&mut self, value: &[u8]) -> Result<()> {
        let length = value.len() as i32;

        self.writer.write_all(&[0x07])?;
        self.writer.write_all(&length.to_be_bytes())?;
        self.writer.write_all(value)?;
        Ok(())
    }

    fn serialize_string(&mut self, value: &str) -> Result<()> {
        let length = value.len() as u16;

        self.writer.write_all(&[0x08])?;
        self.writer.write_all(&length.to_be_bytes())?;
        self.writer.write_all(value.as_bytes())?;
        Ok(())
    }

    fn serialize_list(&mut self, value: &[NbtTag]) -> Result<()> {
        let tag_type = value.first().map_or(0x00, |tag| tag.tag_type());
        let length = value.len() as i32;

        self.writer.write_all(&[0x09])?;
        self.writer.write_all(&tag_type.to_be_bytes())?;
        self.writer.write_all(&length.to_be_bytes())?;
        for tag in value {
            self.serialize_tag(tag)?;
        }
        Ok(())
    }

    fn serialize_compound(&mut self, value: &[(String, NbtTag)]) -> Result<()> {
        self.writer.write_all(&[0x0a])?;
        for (name, tag) in value {
            self.serialize_string(name)?;
            self.serialize_tag(tag)?;
        }
        self.serialize_end()?;
        Ok(())
    }

    fn serialize_int_array(&mut self, value: &[i32]) -> Result<()> {
        let length = value.len() as i32;

        self.writer.write_all(&[0x0b])?;
        self.writer.write_all(&length.to_be_bytes())?;
        for value in value {
            self.writer.write_all(&value.to_be_bytes())?;
        }
        Ok(())
    }

    fn serialize_long_array(&mut self, value: &[i64]) -> Result<()> {
        let length = value.len() as i32;

        self.writer.write_all(&[0x0c])?;
        self.writer.write_all(&length.to_be_bytes())?;
        for value in value {
            self.writer.write_all(&value.to_be_bytes())?;
        }
        Ok(())
    }
}
