use std::{collections::HashMap, io};

use crate::{
    error::{Error, Result},
    nbt_tag::TagType,
    NbtTag,
};

use super::{map_serializer::MapSerializer, serializer::Serializer, unsupported::Unsupported};

pub struct ValueSerializer<'a, W> {
    pub(crate) ser: &'a mut Serializer<W>,
    pub(crate) name: Vec<u8>,
}

impl<'a, W: io::Write> serde::Serializer for &'a mut ValueSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Unsupported;
    type SerializeTuple = Unsupported;
    type SerializeTupleStruct = Unsupported;
    type SerializeTupleVariant = Unsupported;

    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = MapSerializer<'a, W>;
    type SerializeStructVariant = MapSerializer<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.write_byte(if v { 1 } else { 0 })?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.write_byte(v as u8)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.write_short(v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.write_int(v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.write_long(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.write_byte(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.write_short(v as i16)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.write_int(v as i32)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.write_long(v as i64)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.write_float(v)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.write_double(v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.write_byte(v as u8)?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.write_string(v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.write_byte_array(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<()> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(MapSerializer {
            ser: self.ser,
            key: None,
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

/// Headless methods for serializing NBT tags
impl<'a, W: io::Write> ValueSerializer<'a, W> {
    /// Writes a header to the provided tag
    #[inline]
    fn write_header(&mut self, tag_type: TagType, name: &str) -> Result<()> {
        let mut res = vec![tag_type as u8];
        res.extend((name.len() as i16).to_be_bytes());
        res.extend(name.as_bytes());
        self.ser.0.write_all(&res)?;

        Ok(())
    }

    /// Headless version of serialize_byte()
    #[inline]
    fn write_byte(&mut self, v: u8) -> Result<()> {
        self.ser.0.write_all(&[v])?;
        Ok(())
    }

    /// Headless version of serialize_short()
    #[inline]
    fn write_short(&mut self, v: i16) -> Result<()> {
        self.ser.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    /// Headless version of serialize_int()
    #[inline]
    fn write_int(&mut self, v: i32) -> Result<()> {
        self.ser.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    /// Headless version of serialize_long()
    #[inline]
    fn write_long(&mut self, v: i64) -> Result<()> {
        self.ser.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    /// Headless version of serialize_float()
    #[inline]
    fn write_float(&mut self, v: f32) -> Result<()> {
        self.ser.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    /// Headless version of serialize_double()
    #[inline]
    fn write_double(&mut self, v: f64) -> Result<()> {
        self.ser.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    /// Headless version of serialize_byte_array()
    #[inline]
    fn write_byte_array(&mut self, v: &[u8]) -> Result<()> {
        self.write_int(v.len() as i32)?;
        self.ser.0.write_all(v)?;
        Ok(())
    }

    /// Headless version of serialize_string()
    #[inline]
    fn write_string(&mut self, v: &str) -> Result<()> {
        self.write_short(v.len() as i16)?;
        self.ser.0.write_all(v.as_bytes())?;
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
        self.ser.0.write_all(&res)?;

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
        self.ser.0.write_all(&[TagType::End as u8])?;

        Ok(())
    }

    /// Headless version of serialize_int_array()
    fn write_int_array(&mut self, v: &[i32]) -> Result<()> {
        let mut res = (v.len() as i32).to_be_bytes().to_vec();
        for i in v {
            res.extend(i.to_be_bytes());
        }
        self.ser.0.write_all(&res)?;

        Ok(())
    }

    fn write_long_array(&mut self, v: &[i64]) -> Result<()> {
        let mut res = (v.len() as i32).to_be_bytes().to_vec();
        for i in v {
            res.extend(i.to_be_bytes());
        }
        self.ser.0.write_all(&res)?;

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
