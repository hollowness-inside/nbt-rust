use std::{io, collections::HashMap};

use crate::{
    error::{Error, Result},
    NbtTag, nbt_tag::TagType,
};

pub struct Serializer<W>(W);

impl<W: io::Write> Serializer<W> {
    /// Creates a new serializer that writes to the given writer
    pub fn new(writer: W) -> Self {
        Self(writer)
    }

    /// Consumes the serializer and returns the underlying writer
    pub fn into_inner(self) -> W {
        self.0
    }

    /// Serializes a value into NBT
    pub fn serialize<T: Into<NbtTag>>(&mut self, k: &str, v: T) -> Result<()> {
        self.serialize_tag(k, &v.into())
    }

    /// Serializes a tag into NBT
    pub fn serialize_tag(&mut self, k: &str, v: &NbtTag) -> Result<()> {
        match v {
            NbtTag::End => self.serialize_end(),
            NbtTag::Byte(v) => self.serialize_byte(k, *v),
            NbtTag::Short(v) => self.serialize_short(k, *v),
            NbtTag::Int(v) => self.serialize_int(k, *v),
            NbtTag::Long(v) => self.serialize_long(k, *v),
            NbtTag::Float(v) => self.serialize_float(k, *v),
            NbtTag::Double(v) => self.serialize_double(k, *v),
            NbtTag::ByteArray(v) => self.serialize_byte_array(k, v),
            NbtTag::String(v) => self.serialize_string(k, v),
            NbtTag::List(v) => self.serialize_list(k, v),
            NbtTag::Compound(v) => self.serialize_compound(k, v),
            NbtTag::IntArray(v) => self.serialize_int_array(k, v),
            NbtTag::LongArray(v) => self.serialize_long_array(k, v),
        }
    }

    /// Writes the end tag to the underlying writer
    #[inline]
    pub fn serialize_end(&mut self) -> Result<()> {
        self.0.write_all(&[TagType::End as u8])?;
        Ok(())
    }

    /// Serializes a byte into NBT
    pub fn serialize_byte(&mut self, k: &str, v: u8) -> Result<()> {
        self.write_header(TagType::Byte as u8, k)?;
        self.write_byte(v)
    }

    /// Serializes a short into NBT
    pub fn serialize_short(&mut self, k: &str, v: i16) -> Result<()> {
        self.write_header(TagType::Short as u8, k)?;
        self.write_short(v)
    }

    /// Serializes an integer into NBT
    pub fn serialize_int(&mut self, k: &str, v: i32) -> Result<()> {
        self.write_header(TagType::Int as u8, k)?;
        self.write_int(v)
    }

    /// Serializes a long into NBT
    pub fn serialize_long(&mut self, k: &str, v: i64) -> Result<()> {
        self.write_header(TagType::Long as u8, k)?;
        self.write_long(v)
    }

    /// Serializes a float into NBT
    pub fn serialize_float(&mut self, k: &str, v: f32) -> Result<()> {
        self.write_header(TagType::Float as u8, k)?;
        self.write_float(v)
    }

    /// Serializes a double into NBT
    pub fn serialize_double(&mut self, k: &str, v: f64) -> Result<()> {
        self.write_header(TagType::Double as u8, k)?;
        self.write_double(v)
    }

    /// Serializes a byte slice into NBT
    pub fn serialize_byte_array(&mut self, k: &str, v: &[u8]) -> Result<()> {
        self.write_header(TagType::ByteArray as u8, k)?;
        self.write_byte_array(v)
    }

    /// Serializes a string into NBT
    pub fn serialize_string(&mut self, k: &str, v: &str) -> Result<()> {
        self.write_header(TagType::String as u8, k)?;
        self.write_string(v)
    }

    /// Serializes a slice of NBT tags into NBT
    pub fn serialize_list(&mut self, k: &str, value: &[NbtTag]) -> Result<()> {
        self.write_header(TagType::List as u8, k)?;
        self.write_list(value)
    }

    /// Serializes a vector of key-value pairs into NBT
    pub fn serialize_compound(&mut self, k: &str, v: &HashMap<String, NbtTag>) -> Result<()> {
        self.write_header(TagType::Compound as u8, k)?;
        self.write_compound(v)
    }

    /// Serializes a slice of integers into NBT
    pub fn serialize_int_array(&mut self, k: &str, v: &[i32]) -> Result<()> {
        self.write_header(TagType::IntArray as u8, k)?;
        self.write_int_array(v)
    }

    /// Serializes a slice of longs into NBT
    pub fn serialize_long_array(&mut self, k: &str, v: &[i64]) -> Result<()> {
        self.write_header(TagType::LongArray as u8, k)?;
        self.write_long_array(v)
    }

    /// Consumes the serializer and returns a CompoundSerializer
    /// which can be used to serialize a compound tag
    pub fn start_compound(mut self, name: &str) -> Result<CompoundSerializer<W>> {
        self.write_header(TagType::Compound as u8, name)?;
        Ok(CompoundSerializer(self))
    }
}

/// Headless methods for serializing NBT tags
impl <W: io::Write> Serializer<W> {
    /// Writes a header to the provided tag
    #[inline]
    fn write_header(&mut self, prefix: u8, name: &str) -> Result<()> {
        let mut res = vec![prefix];
        res.extend((name.len() as i16).to_be_bytes());
        res.extend(name.as_bytes());
        self.0.write_all(&res)?;

        Ok(())
    }

    /// Headless version of serialize_byte()
    #[inline]
    fn write_byte(&mut self, v: u8) -> Result<()> {
        self.0.write_all(&[v])?;
        Ok(())
    }

    /// Headless version of serialize_short()
    #[inline]
    fn write_short(&mut self, v: i16) -> Result<()> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    /// Headless version of serialize_int()
    #[inline]
    fn write_int(&mut self, v: i32) -> Result<()> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    /// Headless version of serialize_long()
    #[inline]
    fn write_long(&mut self, v: i64) -> Result<()> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    /// Headless version of serialize_float()
    #[inline]
    fn write_float(&mut self, v: f32) -> Result<()> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    /// Headless version of serialize_double()
    #[inline]
    fn write_double(&mut self, v: f64) -> Result<()> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    /// Headless version of serialize_byte_array()
    #[inline]
    fn write_byte_array(&mut self, v: &[u8]) -> Result<()> {
        self.write_int(v.len() as i32)?;
        self.0.write_all(v)?;
        Ok(())
    }

    /// Headless version of serialize_string()
    #[inline]
    fn write_string(&mut self, v: &str) -> Result<()> {
        self.write_short(v.len() as i16)?;
        self.0.write_all(v.as_bytes())?;
        Ok(())
    }

    /// Serializes a slice of NBT tags into NBT
    #[inline]
    fn write_list(&mut self, value: &[NbtTag]) -> Result<()> {
        let Some(tag_type) = value.first().map(|t| t.tag_prefix()) else {
            return Err(Error::EmptySequence);
        };

        if !value.iter().all(|x| x.tag_prefix() == tag_type) {
            return Err(Error::ElementTypesDiffer);
        }

        let mut res = vec![tag_type as u8];
        res.extend((value.len() as i32).to_be_bytes());
        self.0.write_all(&res)?;

        match tag_type {
            TagType::Byte => {
                for i in value {
                    if let NbtTag::Byte(v) = i {
                        self.write_byte(*v)?;
                    }
                }
            }
            TagType::Short => {
                for i in value {
                    if let NbtTag::Short(v) = i {
                        self.write_short(*v)?;
                    }
                }
            }
            TagType::Int => {
                for i in value {
                    if let NbtTag::Int(v) = i {
                        self.write_int(*v)?;
                    }
                }
            }
            TagType::Long => {
                for i in value {
                    if let NbtTag::Long(v) = i {
                        self.write_long(*v)?;
                    }
                }
            }
            TagType::Float => {
                for i in value {
                    if let NbtTag::Float(v) = i {
                        self.write_float(*v)?;
                    }
                }
            }
            TagType::Double => {
                for i in value {
                    if let NbtTag::Double(v) = i {
                        self.write_double(*v)?;
                    }
                }
            }
            TagType::ByteArray => {
                for i in value {
                    if let NbtTag::ByteArray(v) = i {
                        self.write_byte_array(v)?;
                    }
                }
            }
            TagType::String => {
                for i in value {
                    if let NbtTag::String(v) = i {
                        self.write_string(v)?;
                    }
                }
            }
            TagType::List => {
                for i in value {
                    if let NbtTag::List(v) = i {
                        self.write_list(v)?;
                    }
                }
            }
            TagType::Compound => {
                for i in value {
                    if let NbtTag::Compound(v) = i {
                        self.write_compound(&v)?;
                    }
                }
            }
            TagType::IntArray => {
                for i in value {
                    if let NbtTag::IntArray(v) = i {
                        self.write_int_array(v)?;
                    }
                }
            }
            TagType::LongArray => {
                for i in value {
                    if let NbtTag::LongArray(v) = i {
                        self.write_long_array(v)?;
                    }
                }
            }
            _ => {
                return Err(Error::UnknownTagType(tag_type as u8));
            }
        }

        Ok(())
    }

    /// Headless version of serialize_compound()
    fn write_compound(&mut self, v: &HashMap<String, NbtTag>) -> Result<()> {
        for (name, tag) in v {
            self.serialize_tag(&name.to_string(), tag)?;
        }
        self.0.write_all(&[TagType::End as u8])?;

        Ok(())
    }

    /// Headless version of serialize_int_array()
    fn write_int_array(&mut self, v: &[i32]) -> Result<()> {
        let mut res = (v.len() as i32).to_be_bytes().to_vec();
        for i in v {
            res.extend(i.to_be_bytes());
        }
        self.0.write_all(&res)?;

        Ok(())
    }

    fn write_long_array(&mut self, v: &[i64]) -> Result<()> {
        let mut res = (v.len() as i32).to_be_bytes().to_vec();
        for i in v {
            res.extend(i.to_be_bytes());
        }
        self.0.write_all(&res)?;

        Ok(())
    }
}

/// A serializer for compound tags
pub struct CompoundSerializer<W>(Serializer<W>);

impl<W: io::Write> CompoundSerializer<W> {
    /// Serializes a key-value pair into NBT
    pub fn write_field<T: Into<NbtTag>>(&mut self, key: &str, value: T) -> Result<()> {
        self.0.serialize_tag(key, &value.into())
    }

    /// Consumes the compound serializer and returns the underlying Serializer
    pub fn end(mut self) -> Result<Serializer<W>> {
        self.0.serialize_end()?;
        Ok(self.0)
    }
}
