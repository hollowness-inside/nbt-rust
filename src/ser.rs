use std::io::Write;

use serde::Serialize;

use crate::{
    error::{Error, Result},
    nbt_tag::TagType,
};

pub fn to_writer<W, S>(writer: &mut W, value: &S) -> Result<()>
where
    W: Write,
    S: Serialize,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(serializer);
    Ok(())
}

pub struct Serializer<W>(pub(crate) W);

impl<W: Write> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Serializer(writer)
    }

    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<'a, W: Write> serde::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq;
    type SerializeTuple;
    type SerializeTupleStruct;
    type SerializeTupleVariant;
    type SerializeMap;
    type SerializeStruct;
    type SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i8(self, v: i8) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, v: i16) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, v: i32) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i64(self, v: i64) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, v: u16) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, v: u32) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, v: u64) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f64(self, v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_bytes(self, v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(
        self,
        name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        self.0.write_all(&[TagType::Compound as u8]);
        Ok(Self::SerializeMap {
            ser: self,
            key: None,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}
