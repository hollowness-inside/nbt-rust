use std::{collections::HashMap, io, result};

use serde::ser;

use crate::{
    error::{Error, Result},
    nbt_tag::TagType,
    NbtTag,
};

pub struct Serializer<W>(W);

impl<'a, W: io::Write> serde::Serializer for &'a Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer;
    type SerializeTuple = Self::SerializeSeq;
    type SerializeTupleStruct = Self::SerializeTuple;
    type SerializeTupleVariant = Self::SerializeTupleStruct;

    type SerializeMap = MapSerializer;
    type SerializeStruct = Self::SerializeMap;
    type SerializeStructVariant = Self::SerializeStruct;

    fn serialize_bool(self, v: bool) -> result::Result<Self::Ok, Self::Error> {
        self.write_byte(if v { 1 } else { 0 })?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> result::Result<Self::Ok, Self::Error> {
        self.write_byte(v as u8)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> result::Result<Self::Ok, Self::Error> {
        self.write_short(v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> result::Result<Self::Ok, Self::Error> {
        self.write_int(v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> result::Result<Self::Ok, Self::Error> {
        self.write_long(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> result::Result<Self::Ok, Self::Error> {
        self.write_byte(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> result::Result<Self::Ok, Self::Error> {
        self.write_short(v as i16)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> result::Result<Self::Ok, Self::Error> {
        self.write_int(v as i32)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> result::Result<Self::Ok, Self::Error> {
        self.write_long(v as i64)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> result::Result<Self::Ok, Self::Error> {
        self.write_float(v)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> result::Result<Self::Ok, Self::Error> {
        self.write_double(v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> result::Result<Self::Ok, Self::Error> {
        self.write_byte(v as u8)?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> result::Result<Self::Ok, Self::Error> {
        self.write_string(v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> result::Result<Self::Ok, Self::Error> {
        self.write_byte_array(v)?;
        Ok(())
    }

    fn serialize_none(self) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> result::Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> result::Result<Self::Ok, Self::Error> {
        self.write_string(name)?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> result::Result<Self::Ok, Self::Error> {
        self.write_string(variant)?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> result::Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> result::Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.write_header(TagType::String, variant)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> result::Result<Self::SerializeSeq, Self::Error> {
        Ok(Self::SerializeSeq {
            serializer: self,
            len: len.ok_or(Error::UnknownSize)?,
        })
    }

    fn serialize_tuple(self, len: usize) -> result::Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> result::Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> result::Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, len: Option<usize>) -> result::Result<Self::SerializeMap, Self::Error> {
        Ok(Self::SerializeMap { serializer: self })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> result::Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> result::Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

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

pub struct SeqSerializer;

impl ser::SerializeSeq for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> result::Result<(), Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn end(self) -> result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl ser::SerializeTuple for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> result::Result<(), Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn end(self) -> result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl ser::SerializeTupleStruct for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> result::Result<(), Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn end(self) -> result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl ser::SerializeTupleVariant for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> result::Result<(), Self::Error>
    where
        T: serde::Serialize {
        todo!()
    }

    fn end(self) -> result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}