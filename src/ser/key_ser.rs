use crate::error::Error;

use std::io::Write;

use serde::Serialize;

pub struct KeySerializer<'a>(&'a mut [u8]);

impl<'a> KeySerializer<'a> {
    pub fn new(output: &'a mut [u8]) -> Self {
        Self(output)
    }
}

impl<'a, 'k> serde::Serializer for &'a mut KeySerializer<'k> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i8(self, _v: i8) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, _v: i16) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, _v: i32) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i64(self, _v: i64) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u8(self, _v: u8) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, _v: u16) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, _v: u32) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, _v: u64) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, _v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f64(self, _v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_bytes(&[v as u8])
    }

    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        self.0.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> std::result::Result<Self::Ok, Self::Error>
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
        _name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}
