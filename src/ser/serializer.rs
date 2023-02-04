use std::{collections::HashMap, io};

use serde::ser;

use crate::{
    error::{Error, Result},
    nbt_tag::TagType,
    NbtTag,
};

macro_rules! no_name {
    ($name:ident) => {
        fn $name(self) -> Result<()> {
            Err(Error::NoName)
        }
    };
    ($name:ident, $($types:ty),*) => {
        fn $name(self, $(_: $types),*) -> Result<()> {
            Err(Error::NoName)
        }
    };
}

pub struct Serializer<W>(W);

impl<'a, W: io::Write> serde::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer;
    type SerializeTuple = Self::SerializeSeq;
    type SerializeTupleStruct = Self::SerializeTuple;
    type SerializeTupleVariant = Self::SerializeTupleStruct;

    type SerializeMap = MapSerializer;
    type SerializeStruct = Self::SerializeMap;
    type SerializeStructVariant = Self::SerializeStruct;

    no_name!(serialize_bool, bool);
    no_name!(serialize_i8, i8);
    no_name!(serialize_i16, i16);
    no_name!(serialize_i32, i32);
    no_name!(serialize_i64, i64);
    no_name!(serialize_u8, u8);
    no_name!(serialize_u16, u16);
    no_name!(serialize_u32, u32);
    no_name!(serialize_u64, u64);
    no_name!(serialize_f32, f32);
    no_name!(serialize_f64, f64);
    no_name!(serialize_char, char);
    no_name!(serialize_str, &str);
    no_name!(serialize_bytes, &[u8]);
    no_name!(serialize_none);
    no_name!(serialize_unit_variant, &'static str, u32, &'static str);
    no_name!(serialize_unit);
    no_name!(serialize_unit_struct, &'static str);

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::NoName)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
        Err(Error::NoName)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::NoName)
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::NoName)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::NoName)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::NoName)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::NoName)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Self::SerializeMap {
            ser: self,
        })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
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
            self.write_header(tag.tag_type(), name)?;
            self.write_tag(tag.clone())?;
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

    fn write_tag(&mut self, tag: NbtTag) -> Result<()> {
        match tag {
            NbtTag::Byte(v) => self.write_byte(v)?,
            NbtTag::Short(v) => self.write_short(v)?,
            NbtTag::Int(v) => self.write_int(v)?,
            NbtTag::Long(v) => self.write_long(v)?,
            NbtTag::Float(v) => self.write_float(v)?,
            NbtTag::Double(v) => self.write_double(v)?,
            NbtTag::ByteArray(v) => self.write_byte_array(&v)?,
            NbtTag::String(v) => self.write_string(&v)?,
            NbtTag::List(v) => self.write_list(&v)?,
            NbtTag::Compound(v) => self.write_compound(&v)?,
            NbtTag::IntArray(v) => self.write_int_array(&v)?,
            NbtTag::LongArray(v) => self.write_long_array(&v)?,
            NbtTag::End => (),
        }

        Ok(())
    }
}

pub struct SeqSerializer;

impl ser::SerializeSeq for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl ser::SerializeTuple for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl ser::SerializeTupleStruct for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl ser::SerializeTupleVariant for SeqSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

pub struct MapSerializer;

impl ser::SerializeMap for MapSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl ser::SerializeStruct for MapSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl ser::SerializeStructVariant for MapSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}
