use std::io;

use crate::{
    error::{Error, Result},
    nbt_tag::prefixes,
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

    /// Serializes a value into NBT
    pub fn serialize<T: Into<NbtTag>>(&mut self, v: T) -> Result<()> {
        self.serialize_tag(&v.into())
    }

    /// Serializes a tag into NBT
    pub fn serialize_tag(&mut self, v: &NbtTag) -> Result<()> {
        match v {
            NbtTag::End => self.serialize_end(),
            NbtTag::Byte(v) => self.serialize_byte(*v),
            NbtTag::Short(v) => self.serialize_short(*v),
            NbtTag::Int(v) => self.serialize_int(*v),
            NbtTag::Long(v) => self.serialize_long(*v),
            NbtTag::Float(v) => self.serialize_float(*v),
            NbtTag::Double(v) => self.serialize_double(*v),
            NbtTag::ByteArray(v) => self.serialize_byte_array(v),
            NbtTag::String(v) => self.serialize_string(v),
            NbtTag::List(v) => self.serialize_list(v),
            NbtTag::Compound(v) => self.serialize_compound(v),
            NbtTag::IntArray(v) => self.serialize_int_array(v),
            NbtTag::LongArray(v) => self.serialize_long_array(v),
        }
    }

    /// Writes the end tag to the underlying writer
    pub fn serialize_end(&mut self) -> Result<()> {
        self.0.write_all(&[prefixes::END])?;
        Ok(())
    }

    /// Serializes a byte into NBT
    pub fn serialize_byte(&mut self, v: u8) -> Result<()> {
        self.0.write_all(&[prefixes::BYTE, v])?;
        Ok(())
    }

    /// Serializes a short into NBT
    pub fn serialize_short(&mut self, v: i16) -> Result<()> {
        let mut res = vec![prefixes::SHORT];
        res.extend(v.to_be_bytes());
        self.0.write_all(&res)?;
        Ok(())
    }

    /// Serializes an integer into NBT
    pub fn serialize_int(&mut self, v: i32) -> Result<()> {
        let mut res = vec![prefixes::INT];
        res.extend(v.to_be_bytes());
        self.0.write_all(&res)?;

        Ok(())
    }

    /// Serializes a long into NBT
    pub fn serialize_long(&mut self, v: i64) -> Result<()> {
        let mut res = vec![prefixes::LONG];
        res.extend(v.to_be_bytes());
        self.0.write_all(&res)?;

        Ok(())
    }

    /// Serializes a float into NBT
    pub fn serialize_float(&mut self, v: f32) -> Result<()> {
        let mut res = vec![prefixes::FLOAT];
        res.extend(v.to_be_bytes());
        self.0.write_all(&res)?;

        Ok(())
    }

    /// Serializes a double into NBT
    pub fn serialize_double(&mut self, v: f64) -> Result<()> {
        let mut res = vec![prefixes::DOUBLE];
        res.extend(v.to_be_bytes());
        self.0.write_all(&res)?;

        Ok(())
    }

    /// Serializes a byte slice into NBT
    pub fn serialize_byte_array(&mut self, v: &[u8]) -> Result<()> {
        let mut res = vec![prefixes::BYTE_ARRAY];
        res.extend((v.len() as i32).to_be_bytes());
        res.extend(v);
        self.0.write_all(&res)?;

        Ok(())
    }

    /// Serializes a string into NBT
    pub fn serialize_string(&mut self, v: &str) -> Result<()> {
        let mut res = vec![prefixes::STRING];
        res.extend((v.len() as i16).to_be_bytes());
        res.extend(v.as_bytes());
        self.0.write_all(&res)?;

        Ok(())
    }

    /// Serializes a slice of NBT tags into NBT
    pub fn serialize_list(&mut self, value: &[NbtTag]) -> Result<()> {
        let Some(tag_type) = value.first().map(|t| t.tag_type()) else {
            return Err(Error::Generic("Cannot serialize an empty list".to_string()));
        };

        if !value.iter().all(|x| x.tag_type() == tag_type) {
            return Err(Error::Generic(
                "All elements in a list must have the same type".to_string(),
            ));
        }

        let mut res = vec![prefixes::LIST];
        res.push(tag_type);
        res.extend((value.len() as i32).to_be_bytes());
        self.0.write_all(&res)?;

        match tag_type {
            prefixes::BYTE => {
                for i in value {
                    if let NbtTag::Byte(v) = i {
                        self.serialize_byte(*v)?;
                    }
                }
            }
            prefixes::SHORT => {
                for i in value {
                    if let NbtTag::Short(v) = i {
                        self.serialize_short(*v)?;
                    }
                }
            }
            prefixes::INT => {
                for i in value {
                    if let NbtTag::Int(v) = i {
                        self.serialize_int(*v)?;
                    }
                }
            }
            prefixes::LONG => {
                for i in value {
                    if let NbtTag::Long(v) = i {
                        self.serialize_long(*v)?;
                    }
                }
            }
            prefixes::FLOAT => {
                for i in value {
                    if let NbtTag::Float(v) = i {
                        self.serialize_float(*v)?;
                    }
                }
            }
            prefixes::DOUBLE => {
                for i in value {
                    if let NbtTag::Double(v) = i {
                        self.serialize_double(*v)?;
                    }
                }
            }
            prefixes::BYTE_ARRAY => {
                for i in value {
                    if let NbtTag::ByteArray(v) = i {
                        self.serialize_byte_array(v)?;
                    }
                }
            }
            prefixes::STRING => {
                for i in value {
                    if let NbtTag::String(v) = i {
                        self.serialize_string(v)?;
                    }
                }
            }
            prefixes::LIST => {
                for i in value {
                    if let NbtTag::List(v) = i {
                        self.serialize_list(v)?;
                    }
                }
            }
            prefixes::COMPOUND => {
                for i in value {
                    if let NbtTag::Compound(v) = i {
                        self.serialize_compound(v)?;
                    }
                }
            }
            prefixes::INT_ARRAY => {
                for i in value {
                    if let NbtTag::IntArray(v) = i {
                        self.serialize_int_array(v)?;
                    }
                }
            }
            prefixes::LONG_ARRAY => {
                for i in value {
                    if let NbtTag::LongArray(v) = i {
                        self.serialize_long_array(v)?;
                    }
                }
            }
            _ => {
                return Err(Error::Generic(format!(
                    "Unknown tag type: {}",
                    tag_type
                )));
            }
        }

        Ok(())
    }

    /// Serializes a vector of key-value pairs into NBT
    pub fn serialize_compound<S: ToString>(&mut self, v: &[(S, NbtTag)]) -> Result<()> {
        self.0.write_all(&[prefixes::COMPOUND])?;

        for (name, tag) in v {
            self.serialize_string(&name.to_string())?;
            self.serialize_tag(tag)?;
        }
        self.0.write_all(&[prefixes::END])?;

        Ok(())
    }

    /// Serializes a slice of integers into NBT
    pub fn serialize_int_array(&mut self, v: &[i32]) -> Result<()> {
        let mut res = vec![prefixes::INT_ARRAY];
        res.extend((v.len() as i32).to_be_bytes());
        for i in v {
            res.extend(i.to_be_bytes());
        }
        self.0.write_all(&res)?;

        Ok(())
    }

    /// Serializes a slice of longs into NBT
    pub fn serialize_long_array(&mut self, v: &[i64]) -> Result<()> {
        let mut res = vec![prefixes::LONG_ARRAY];
        res.extend((v.len() as i32).to_be_bytes());
        for i in v {
            res.extend(i.to_be_bytes());
        }
        self.0.write_all(&res)?;

        Ok(())
    }

    /// Consumes the serializer and returns a CompoundSerializer
    /// which can be used to serialize a compound tag
    pub fn start_compound(self) -> CompoundSerializer<W> {
        CompoundSerializer {
            ser: self,
            is_first: true,
        }
    }
}

/// A serializer for compound tags
pub struct CompoundSerializer<W> {
    ser: Serializer<W>,
    is_first: bool,
}

impl<W: io::Write> CompoundSerializer<W> {
    /// Serializes a key-value pair into NBT
    pub fn write_field<T: Into<NbtTag>>(&mut self, key: &str, value: T) -> Result<()> {
        if self.is_first {
            self.ser.0.write_all(&[prefixes::COMPOUND])?;
            self.is_first = false;
        }

        self.ser.serialize_string(key)?;
        self.ser.serialize(value)?;

        Ok(())
    }

    /// Consumes the compound serializer and returns the underlying Serializer
    pub fn end(mut self) -> Result<Serializer<W>> {
        self.ser.serialize_end()?;
        Ok(self.ser)
    }
}
