use crate::error::Error;

use std::io::Write;

use serde::Serialize;

use super::unsupported::unsupported;

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

    fn serialize_bytes(self, v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        self.0.write(v)?;
        Ok(())
    }
    
    #[inline]
    fn serialize_char(self, v: char) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_bytes(&[v as u8])
    }

    #[inline]
    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
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
    unsupported!(serialize_unit);
    unsupported!(serialize_unit_struct, &'static str);
    unsupported!(serialize_unit_variant, &'static str, u32, &'static str);

    unsupported!(serialize_seq -> SerializeSeq, Option<usize>);
    unsupported!(serialize_tuple -> SerializeTuple, usize);
    unsupported!(serialize_tuple_struct -> SerializeTupleStruct, &'static str, usize);
    unsupported!(serialize_tuple_variant -> SerializeTupleVariant, &'static str, u32, &'static str, usize);
    unsupported!(serialize_map -> SerializeMap, Option<usize>);
    unsupported!(serialize_struct -> SerializeMap, &'static str, usize);
    unsupported!(serialize_struct_variant -> SerializeMap, &'static str, u32, &'static str, usize);

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::Unsupported("serialize_newtype_struct".to_string()))
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
        Err(Error::Unsupported("serialize_newtype_variant".to_string()))
    }
}
