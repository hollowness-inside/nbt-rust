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
        self.serialize_i8(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.write_header(TagType::Byte, &self.name.clone())?;
        self.ser.0.write_all(&[v as u8])?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.write_header(TagType::Short, &self.name.clone())?;
        self.ser.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.write_header(TagType::Int, &self.name.clone())?;
        self.ser.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.write_header(TagType::Long, &self.name.clone())?;
        self.ser.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_i8(v as i8)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_i16(v as i16)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_i32(v as i32)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.write_header(TagType::Float, &self.name.clone())?;
        self.ser.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.write_header(TagType::Double, &self.name.clone())?;
        self.ser.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_i8(v as i8)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.write_header(TagType::String, &self.name.clone())?;
        self.ser.0.write_all(&(v.len() as i16).to_be_bytes())?;
        self.ser.0.write_all(v.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.write_header(TagType::ByteArray, &self.name.clone())?;
        self.ser.0.write_all(&(v.len() as i32).to_be_bytes())?;
        self.ser.0.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
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
        Err(Error::Unsupported)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Unsupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.write_header(TagType::Compound, &self.name.clone())?;
        Ok(MapSerializer {
            ser: self.ser,
            key: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(None)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::Unsupported)
    }
}

/// Headless methods for serializing NBT tags
impl<'a, W: io::Write> ValueSerializer<'a, W> {
    /// Writes a header to the provided tag
    #[inline]
    fn write_header(&mut self, tag_type: TagType, name: &[u8]) -> Result<()> {
        let mut res = vec![tag_type as u8];
        res.extend((name.len() as i16).to_be_bytes());
        res.extend(name);
        self.ser.0.write_all(&res)?;

        Ok(())
    }
}
