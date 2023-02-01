use std::{collections::HashMap, io};

use crate::{
    error::{Error, Result},
    nbt_tag::TagType,
    NbtTag,
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
}

/// Headless methods for serializing NBT tags
impl<W: io::Write> Serializer<W> {
    /// Writes a header to the provided tag
    #[inline]
    fn write_header(&mut self, tag_type: TagType, name: &str) -> Result<()> {
        let mut res = vec![tag_type as u8];
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
        let Some(tag_type) = value.first().map(|t| t.tag_type()) else {
            return Err(Error::EmptySequence);
        };

        if !value.iter().all(|x| x.tag_type() == tag_type) {
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
                        self.write_compound(v)?;
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
