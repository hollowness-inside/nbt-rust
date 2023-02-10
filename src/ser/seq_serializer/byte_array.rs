use std::io;

use crate::{
    error::{Error, Result},
    ser::{unsupported::unsupported, Serializer, Unsupported},
};

pub struct ByteArraySerializer<'a, W> {
    pub(crate) ser: &'a mut Serializer<W>,
    pub(crate) len: usize,
}

impl<'a, W: io::Write> serde::ser::SerializeSeq for ByteArraySerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let mut out = Vec::new();
        value.serialize(&mut ByteSerializer(&mut out))
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        unimplemented!("Type of sequence must be specified")
    }
}

pub struct ByteSerializer<W>(W);

impl<'a, W: io::Write> serde::Serializer for &'a mut ByteSerializer<W> {
    type Ok = ();
    type Error = Error;

    /// We only need to serialize strings and bytes, so all these
    /// methods are not necessary and marked as unsupported.
    type SerializeSeq = Unsupported;
    type SerializeTuple = Unsupported;
    type SerializeTupleStruct = Unsupported;
    type SerializeTupleVariant = Unsupported;
    type SerializeMap = Unsupported;
    type SerializeStruct = Unsupported;
    type SerializeStructVariant = Unsupported;

    unsupported!(serialize_bytes, &[u8]);
    unsupported!(serialize_str, &str);
    unsupported!(serialize_char, char);
    unsupported!(serialize_bool, bool);
    unsupported!(serialize_i8, i8);
    unsupported!(serialize_i16, i16);
    unsupported!(serialize_i32, i32);
    unsupported!(serialize_i64, i64);
    unsupported!(serialize_u8, u8);
    unsupported!(serialize_u16, u16);
    unsupported!(serialize_u32, u32);
    unsupported!(serialize_u64, u64);
    unsupported!(serialize_f32, f32);
    unsupported!(serialize_f64, f64);
    unsupported!(serialize_none);
    unsupported!(serialize_unit_struct, &'static str);
    unsupported!(serialize_unit_variant, &'static str, u32, &'static str);
    unsupported!(serialize_unit);
    unsupported!(serialize_seq -> SerializeSeq, Option<usize>);
    unsupported!(serialize_tuple -> SerializeTuple, usize);
    unsupported!(serialize_tuple_struct -> SerializeTupleStruct, &'static str, usize);
    unsupported!(serialize_tuple_variant -> SerializeTupleVariant, &'static str, u32, &'static str, usize);
    unsupported!(serialize_map -> SerializeMap, Option<usize>);
    unsupported!(serialize_struct -> SerializeStruct, &'static str, usize);
    unsupported!(serialize_struct_variant -> SerializeStructVariant, &'static str, u32, &'static str, usize);

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
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
        unimplemented!("KeySerializer::serialize_newtype_variant")
    }
}
