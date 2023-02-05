use std::io;

use super::unsupported::{unsupported, Unsupported};
use crate::error::{Error, Result};

use serde::ser::Serializer;

pub(crate) struct KeySerializer<W> {
    pub(crate) ser: W,
}

impl<W> KeySerializer<W> {
    pub fn new(writer: W) -> Self {
        KeySerializer { ser: writer }
    }
}

impl<'a, W: io::Write> Serializer for &'a mut KeySerializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Unsupported;
    type SerializeTuple = Unsupported;
    type SerializeTupleStruct = Unsupported;
    type SerializeTupleVariant = Unsupported;
    type SerializeMap = Unsupported;
    type SerializeStruct = Unsupported;
    type SerializeStructVariant = Unsupported;

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

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

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
            "KeySerializer::serialize_newtype_variant".to_string(),
        ))
    }
}
