use std::io;

use crate::{NbtTag, error::Result};

pub struct Serializer<W> {
    writer: W
}

impl<W: io::Write> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }

    pub fn serialize<T: Into<NbtTag>>(&mut self, v: T) -> Result<()> {
        self.serialize_tag(&v.into())
    }

    pub fn serialize_tag(&mut self, tag: &NbtTag) -> Result<()> {
        match tag {
            NbtTag::End => self.serialize_end(),
            NbtTag::Byte(value) => self.serialize_byte(*value),
            NbtTag::Short(value) => self.serialize_short(*value),
            NbtTag::Int(value) => self.serialize_int(*value),
            NbtTag::Long(value) => self.serialize_long(*value),
            NbtTag::Float(value) => self.serialize_float(*value),
            NbtTag::Double(value) => self.serialize_double(*value),
            NbtTag::ByteArray(value) => self.serialize_byte_array(value),
            NbtTag::String(value) => self.serialize_string(value),
            NbtTag::List(value) => self.serialize_list(value),
            NbtTag::Compound(value) => self.serialize_compound(value),
            NbtTag::IntArray(value) => self.serialize_int_array(value),
            NbtTag::LongArray(value) => self.serialize_long_array(value),
        }
    }

    pub fn serialize_end(&mut self) -> Result<()> {
        self.writer.write_all(&[0x00])?;
        Ok(())
    }

    pub fn serialize_byte(&mut self, value: u8) -> Result<()> {
        self.writer.write_all(&[0x01])?;
        self.writer.write_all(&[value])?;
        Ok(())
    }

    pub fn serialize_short(&mut self, value: i16) -> Result<()> {
        self.writer.write_all(&[0x02])?;
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    pub fn serialize_int(&mut self, value: i32) -> Result<()> {
        self.writer.write_all(&[0x03])?;
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    pub fn serialize_long(&mut self, value: i64) -> Result<()> {
        self.writer.write_all(&[0x04])?;
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    pub fn serialize_float(&mut self, value: f32) -> Result<()> {
        self.writer.write_all(&[0x05])?;
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    pub fn serialize_double(&mut self, value: f64) -> Result<()> {
        self.writer.write_all(&[0x06])?;
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    pub fn serialize_byte_array(&mut self, value: &[u8]) -> Result<()> {
        self.writer.write_all(&[0x07])?;
        self.writer.write_all(&(value.len() as i32).to_be_bytes())?;
        self.writer.write_all(value)?;
        Ok(())
    }

    pub fn serialize_string(&mut self, value: &str) -> Result<()> {
        self.writer.write_all(&[0x08])?;
        self.writer.write_all(&(value.len() as i16).to_be_bytes())?;
        self.writer.write_all(value.as_bytes())?;
        Ok(())
    }

    pub fn serialize_list(&mut self, value: &[NbtTag]) -> Result<()> {
        self.writer.write_all(&[0x09])?;
        self.writer.write_all(&value[0].tag_type().to_be_bytes())?;
        self.writer.write_all(&(value.len() as i32).to_be_bytes())?;
        for tag in value {
            self.serialize_tag(tag)?;
        }
        Ok(())
    }

    pub fn serialize_compound(&mut self, value: &[(String, NbtTag)]) -> Result<()> {
        self.writer.write_all(&[0x0a])?;
        for (name, tag) in value {
            self.serialize_string(name)?;
            self.serialize_tag(tag)?;
        }
        self.serialize_end()?;
        Ok(())
    }

    pub fn serialize_int_array(&mut self, value: &[i32]) -> Result<()> {
        self.writer.write_all(&[0x0b])?;
        self.writer.write_all(&(value.len() as i32).to_be_bytes())?;
        for &int in value {
            self.writer.write_all(&int.to_be_bytes())?;
        }
        Ok(())
    }

    pub fn serialize_long_array(&mut self, value: &[i64]) -> Result<()> {
        self.writer.write_all(&[0x0c])?;
        self.writer.write_all(&(value.len() as i32).to_be_bytes())?;
        for &long in value {
            self.writer.write_all(&long.to_be_bytes())?;
        }
        Ok(())
    }
}

impl From<u32> for NbtTag {
    fn from(value: u32) -> Self {
        NbtTag::Int(value as i32)
    }
}

impl From<i32> for NbtTag {
    fn from(value: i32) -> Self {
        NbtTag::Int(value)
    }
}

impl From<u64> for NbtTag {
    fn from(value: u64) -> Self {
        NbtTag::Long(value as i64)
    }
}

impl From<i64> for NbtTag {
    fn from(value: i64) -> Self {
        NbtTag::Long(value)
    }
}

impl From<f32> for NbtTag {
    fn from(value: f32) -> Self {
        NbtTag::Float(value)
    }
}

impl From<f64> for NbtTag {
    fn from(value: f64) -> Self {
        NbtTag::Double(value)
    }
}

impl From<&str> for NbtTag {
    fn from(value: &str) -> Self {
        NbtTag::String(value.to_string())
    }
}

impl From<String> for NbtTag {
    fn from(value: String) -> Self {
        NbtTag::String(value)
    }
}

impl From<Vec<u8>> for NbtTag {
    fn from(value: Vec<u8>) -> Self {
        NbtTag::ByteArray(value)
    }
}

impl From<&[u8]> for NbtTag {
    fn from(value: &[u8]) -> Self {
        NbtTag::ByteArray(value.to_vec())
    }
}

impl From<Vec<i32>> for NbtTag {
    fn from(value: Vec<i32>) -> Self {
        NbtTag::IntArray(value)
    }
}

impl From<&[i32]> for NbtTag {
    fn from(value: &[i32]) -> Self {
        NbtTag::IntArray(value.to_vec())
    }
}

impl From<Vec<i64>> for NbtTag {
    fn from(value: Vec<i64>) -> Self {
        NbtTag::LongArray(value)
    }
}

impl From<&[i64]> for NbtTag {
    fn from(value: &[i64]) -> Self {
        NbtTag::LongArray(value.to_vec())
    }
}

impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
}

    fn end(self) -> Result<()> {
        self.writer.write_all(&[prefixes::END])?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
}

    fn end(self) -> Result<()> {
        self.writer.write_all(&[prefixes::END])?;
        Ok(())
    }
}