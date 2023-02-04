use std::{collections::HashMap, io};

use crate::{
    error::{Error, Result},
    nbt_tag::TagType,
    NbtTag,
};

use super::{map_serializer::MapSerializer, unsupported::Unsupported};

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

    type SerializeSeq = Unsupported;
    type SerializeTuple = Unsupported;
    type SerializeTupleStruct = Unsupported;
    type SerializeTupleVariant = Unsupported;

    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = MapSerializer<'a, W>;
    type SerializeStructVariant = MapSerializer<'a, W>;

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
            key: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
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