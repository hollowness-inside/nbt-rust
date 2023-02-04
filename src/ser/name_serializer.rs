use std::io;

use super::unsupported::Unsupported;
use crate::error::{Error, Result};

use serde::ser::Serializer;

pub struct NameSerializer<W>(W);

impl<W> NameSerializer<W> {
    pub fn new(writer: W) -> Self {
        NameSerializer(writer)
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

    fn serialize_bool(self, v: bool) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.0.write_all(&[v as u8])?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.0.write_all(v.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.0.write_all(v)?;
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
        self.0.write_all(&[0, 0])?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<()>
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
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        Err(Error::Unsupported)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::Unsupported)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Unsupported)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::Unsupported)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::Unsupported)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::Unsupported)
    }
}
