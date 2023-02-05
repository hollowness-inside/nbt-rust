use std::io;

use super::unsupported::Unsupported;
use crate::error::{Error, Result};

use serde::ser::Serializer;

pub struct NameSerializer<W> {
    pub(crate) ser: W,
}

impl<W> NameSerializer<W> {
    pub fn new(writer: W) -> Self {
        NameSerializer { ser: writer }
    }
}

impl<'a, W: io::Write> Serializer for &'a mut NameSerializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Unsupported;
    type SerializeTuple = Unsupported;
    type SerializeTupleStruct = Unsupported;
    type SerializeTupleVariant = Unsupported;
    type SerializeMap = Unsupported;
    type SerializeStruct = Unsupported;
    type SerializeStructVariant = Unsupported;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_bool".to_string(),
        ))
    }

    fn serialize_i8(self, _v: i8) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_i8".to_string(),
        ))
    }

    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_i16".to_string(),
        ))
    }

    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_i32".to_string(),
        ))
    }

    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_i64".to_string(),
        ))
    }

    fn serialize_u8(self, _v: u8) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_u8".to_string(),
        ))
    }

    fn serialize_u16(self, _v: u16) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_u16".to_string(),
        ))
    }

    fn serialize_u32(self, _v: u32) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_u32".to_string(),
        ))
    }

    fn serialize_u64(self, _v: u64) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_u64".to_string(),
        ))
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_f32".to_string(),
        ))
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_f64".to_string(),
        ))
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.ser.write_all(&[v as u8])?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.ser.write_all(v.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.ser.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_none".to_string(),
        ))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.ser.write_all(&[0, 0])?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_unit_struct".to_string(),
        ))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_unit_variant".to_string(),
        ))
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
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_newtype_variant".to_string(),
        ))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_seq".to_string(),
        ))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_tuple".to_string(),
        ))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_tuple_struct".to_string(),
        ))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_tuple_variant".to_string(),
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_map".to_string(),
        ))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_struct".to_string(),
        ))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::UnsupportedMethod(
            "NameSerializer::serialize_struct_variant".to_string(),
        ))
    }
}
